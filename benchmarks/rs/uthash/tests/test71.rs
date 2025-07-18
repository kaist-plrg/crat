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
    printf(b"LL prepend elem\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp: *mut el = 0 as *mut el;
    els[0 as libc::c_int as usize].next = 0 as *mut el;
    if !headA.is_null() {
        _tmp = headA;
        while !((*_tmp).next).is_null() {
            _tmp = (*_tmp).next;
        }
        (*_tmp)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_0: *mut el = 0 as *mut el;
    els[1 as libc::c_int as usize].next = 0 as *mut el;
    if !headA.is_null() {
        _tmp_0 = headA;
        while !((*_tmp_0).next).is_null() {
            _tmp_0 = (*_tmp_0).next;
        }
        (*_tmp_0)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_1: *mut el = 0 as *mut el;
    els[2 as libc::c_int as usize].next = 0 as *mut el;
    if !headA.is_null() {
        _tmp_1 = headA;
        while !((*_tmp_1).next).is_null() {
            _tmp_1 = (*_tmp_1).next;
        }
        (*_tmp_1)
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_2: *mut el = 0 as *mut el;
    els[3 as libc::c_int as usize].next = 0 as *mut el;
    if !headA.is_null() {
        _tmp_2 = headA;
        while !((*_tmp_2).next).is_null() {
            _tmp_2 = (*_tmp_2).next;
        }
        (*_tmp_2)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_3: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_4233: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_4183: {
            if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headA == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_3 = headA;
            while !((*_tmp_3).next).is_null()
                && (*_tmp_3).next
                    != &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_3 = (*_tmp_3).next;
            }
            if !((*_tmp_3).next).is_null() {
                (*_tmp_3)
                    .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_4: *mut el = 0 as *mut el;
        els[4 as libc::c_int as usize].next = 0 as *mut el;
        if !headA.is_null() {
            _tmp_4 = headA;
            while !((*_tmp_4).next).is_null() {
                _tmp_4 = (*_tmp_4).next;
            }
            (*_tmp_4)
                .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_5: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3940: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3890: {
            if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headA == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_5 = headA;
            while !((*_tmp_5).next).is_null()
                && (*_tmp_5).next
                    != &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_5 = (*_tmp_5).next;
            }
            if !((*_tmp_5).next).is_null() {
                (*_tmp_5)
                    .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_6: *mut el = 0 as *mut el;
        els[5 as libc::c_int as usize].next = 0 as *mut el;
        if !headA.is_null() {
            _tmp_6 = headA;
            while !((*_tmp_6).next).is_null() {
                _tmp_6 = (*_tmp_6).next;
            }
            (*_tmp_6)
                .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_7: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3647: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3597: {
            if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headA == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_7 = headA;
            while !((*_tmp_7).next).is_null()
                && (*_tmp_7).next
                    != &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_7 = (*_tmp_7).next;
            }
            if !((*_tmp_7).next).is_null() {
                (*_tmp_7)
                    .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_8: *mut el = 0 as *mut el;
        els[6 as libc::c_int as usize].next = 0 as *mut el;
        if !headA.is_null() {
            _tmp_8 = headA;
            while !((*_tmp_8).next).is_null() {
                _tmp_8 = (*_tmp_8).next;
            }
            (*_tmp_8)
                .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_9: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3354: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3304: {
            if !(&mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[7]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headA == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_9 = headA;
            while !((*_tmp_9).next).is_null()
                && (*_tmp_9).next
                    != &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_9 = (*_tmp_9).next;
            }
            if !((*_tmp_9).next).is_null() {
                (*_tmp_9)
                    .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_10: *mut el = 0 as *mut el;
        els[7 as libc::c_int as usize].next = 0 as *mut el;
        if !headA.is_null() {
            _tmp_10 = headA;
            while !((*_tmp_10).next).is_null() {
                _tmp_10 = (*_tmp_10).next;
            }
            (*_tmp_10)
                .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize)
                as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_11: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3061: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3011: {
            if !(&mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[8]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headA == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_11 = headA;
            while !((*_tmp_11).next).is_null()
                && (*_tmp_11).next
                    != &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_11 = (*_tmp_11).next;
            }
            if !((*_tmp_11).next).is_null() {
                (*_tmp_11)
                    .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_12: *mut el = 0 as *mut el;
        els[8 as libc::c_int as usize].next = 0 as *mut el;
        if !headA.is_null() {
            _tmp_12 = headA;
            while !((*_tmp_12).next).is_null() {
                _tmp_12 = (*_tmp_12).next;
            }
            (*_tmp_12)
                .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        }
    }
    if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_13: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2801: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2751: {
            if !(&mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[9]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headA == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_13 = headA;
            while !((*_tmp_13).next).is_null()
                && (*_tmp_13).next
                    != &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_13 = (*_tmp_13).next;
            }
            if !((*_tmp_13).next).is_null() {
                (*_tmp_13)
                    .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_14: *mut el = 0 as *mut el;
        els[9 as libc::c_int as usize].next = 0 as *mut el;
        if !headA.is_null() {
            _tmp_14 = headA;
            while !((*_tmp_14).next).is_null() {
                _tmp_14 = (*_tmp_14).next;
            }
            (*_tmp_14)
                .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize)
                as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        }
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
            let mut _tmp_15: *mut el = 0 as *mut el;
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
            'c_2482: {
                if !headA.is_null() {} else {
                    __assert_fail(
                        b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test71.c\0" as *const u8 as *const libc::c_char,
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
                    b"test71.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
            'c_2429: {
                if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
                {} else {
                    __assert_fail(
                        b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test71.c\0" as *const u8 as *const libc::c_char,
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
            if headA == e {
                headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            } else {
                _tmp_15 = headA;
                while !((*_tmp_15).next).is_null() && (*_tmp_15).next != e {
                    _tmp_15 = (*_tmp_15).next;
                }
                if !((*_tmp_15).next).is_null() {
                    (*_tmp_15)
                        .next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                }
            }
        } else {
            let mut _tmp_16: *mut el = 0 as *mut el;
            els[i as usize].next = 0 as *mut el;
            if !headA.is_null() {
                _tmp_16 = headA;
                while !((*_tmp_16).next).is_null() {
                    _tmp_16 = (*_tmp_16).next;
                }
                (*_tmp_16).next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            } else {
                headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            }
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
    let mut _tmp_17: *mut el = 0 as *mut el;
    els[20 as libc::c_int as usize].next = 0 as *mut el;
    if !headB.is_null() {
        _tmp_17 = headB;
        while !((*_tmp_17).next).is_null() {
            _tmp_17 = (*_tmp_17).next;
        }
        (*_tmp_17)
            .next = &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el;
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el).is_null()
    {
        let mut _tmp_18: *mut el = 0 as *mut el;
        if !headB.is_null() {} else {
            __assert_fail(
                b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2079: {
            if !headB.is_null() {} else {
                __assert_fail(
                    b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2029: {
            if !(&mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[21]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headB == &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el
        {
            headB = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_18 = headB;
            while !((*_tmp_18).next).is_null()
                && (*_tmp_18).next
                    != &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_18 = (*_tmp_18).next;
            }
            if !((*_tmp_18).next).is_null() {
                (*_tmp_18)
                    .next = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_19: *mut el = 0 as *mut el;
        els[21 as libc::c_int as usize].next = 0 as *mut el;
        if !headB.is_null() {
            _tmp_19 = headB;
            while !((*_tmp_19).next).is_null() {
                _tmp_19 = (*_tmp_19).next;
            }
            (*_tmp_19)
                .next = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize)
                as *mut el;
        } else {
            headB = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        }
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el).is_null()
    {
        let mut _tmp_20: *mut el = 0 as *mut el;
        if !headB.is_null() {} else {
            __assert_fail(
                b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                b"test71.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1785: {
            if !headB.is_null() {} else {
                __assert_fail(
                    b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
                b"test71.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1730: {
            if !(&mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[22]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test71.c\0" as *const u8 as *const libc::c_char,
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
        if headB == &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el
        {
            headB = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_20 = headB;
            while !((*_tmp_20).next).is_null()
                && (*_tmp_20).next
                    != &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_20 = (*_tmp_20).next;
            }
            if !((*_tmp_20).next).is_null() {
                (*_tmp_20)
                    .next = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_21: *mut el = 0 as *mut el;
        els[22 as libc::c_int as usize].next = 0 as *mut el;
        if !headB.is_null() {
            _tmp_21 = headB;
            while !((*_tmp_21).next).is_null() {
                _tmp_21 = (*_tmp_21).next;
            }
            (*_tmp_21)
                .next = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize)
                as *mut el;
        } else {
            headB = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        }
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
