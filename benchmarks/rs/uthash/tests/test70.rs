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
    let mut headA: *mut el = 0 as *mut el;
    let mut headB: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        i += 1;
        i;
    }
    printf(b"LL replace elem\n\0" as *const u8 as *const libc::c_char);
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
    let mut _tmp_3: *mut el = 0 as *mut el;
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3784: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3734: {
        if !(&mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[0]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3684: {
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[4 as libc::c_int as usize].next = els[0 as libc::c_int as usize].next;
    if headA == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
        headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_3 = headA;
        while !((*_tmp_3).next).is_null()
            && (*_tmp_3).next
                != &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
        {
            _tmp_3 = (*_tmp_3).next;
        }
        if !((*_tmp_3).next).is_null() {
            (*_tmp_3)
                .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_4: *mut el = 0 as *mut el;
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3510: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3460: {
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3410: {
        if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[5 as libc::c_int as usize].next = els[4 as libc::c_int as usize].next;
    if headA == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el {
        headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_4 = headA;
        while !((*_tmp_4).next).is_null()
            && (*_tmp_4).next
                != &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el
        {
            _tmp_4 = (*_tmp_4).next;
        }
        if !((*_tmp_4).next).is_null() {
            (*_tmp_4)
                .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_5: *mut el = 0 as *mut el;
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3236: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3186: {
        if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3136: {
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[6 as libc::c_int as usize].next = els[3 as libc::c_int as usize].next;
    if headA == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el {
        headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_5 = headA;
        while !((*_tmp_5).next).is_null()
            && (*_tmp_5).next
                != &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el
        {
            _tmp_5 = (*_tmp_5).next;
        }
        if !((*_tmp_5).next).is_null() {
            (*_tmp_5)
                .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_6: *mut el = 0 as *mut el;
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2962: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2912: {
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2862: {
        if !(&mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[7]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[7 as libc::c_int as usize].next = els[6 as libc::c_int as usize].next;
    if headA == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el {
        headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_6 = headA;
        while !((*_tmp_6).next).is_null()
            && (*_tmp_6).next
                != &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el
        {
            _tmp_6 = (*_tmp_6).next;
        }
        if !((*_tmp_6).next).is_null() {
            (*_tmp_6)
                .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize)
                as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_7: *mut el = 0 as *mut el;
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2688: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2638: {
        if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[1]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2588: {
        if !(&mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[8]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[8 as libc::c_int as usize].next = els[1 as libc::c_int as usize].next;
    if headA == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
        headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_7 = headA;
        while !((*_tmp_7).next).is_null()
            && (*_tmp_7).next
                != &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
        {
            _tmp_7 = (*_tmp_7).next;
        }
        if !((*_tmp_7).next).is_null() {
            (*_tmp_7)
                .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut el;
        }
    }
    let mut _tmp_8: *mut el = 0 as *mut el;
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2447: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2397: {
        if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[2]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2347: {
        if !(&mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[9]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[9 as libc::c_int as usize].next = els[2 as libc::c_int as usize].next;
    if headA == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
        headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_8 = headA;
        while !((*_tmp_8).next).is_null()
            && (*_tmp_8).next
                != &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
        {
            _tmp_8 = (*_tmp_8).next;
        }
        if !((*_tmp_8).next).is_null() {
            (*_tmp_8)
                .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize)
                as *mut el;
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
        let mut _tmp_9: *mut el = 0 as *mut el;
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2144: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test70.c\0" as *const u8 as *const libc::c_char,
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
                b"test70.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2099: {
            if !e.is_null() {} else {
                __assert_fail(
                    b"(e) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test70.c\0" as *const u8 as *const libc::c_char,
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
                b"test70.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2046: {
            if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
            {} else {
                __assert_fail(
                    b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test70.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[i as usize].next = (*e).next;
        if headA == e {
            headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
        } else {
            _tmp_9 = headA;
            while !((*_tmp_9).next).is_null() && (*_tmp_9).next != e {
                _tmp_9 = (*_tmp_9).next;
            }
            if !((*_tmp_9).next).is_null() {
                (*_tmp_9).next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
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
    let mut _tmp_10: *mut el = 0 as *mut el;
    els[18 as libc::c_int as usize].next = 0 as *mut el;
    if !headB.is_null() {
        _tmp_10 = headB;
        while !((*_tmp_10).next).is_null() {
            _tmp_10 = (*_tmp_10).next;
        }
        (*_tmp_10)
            .next = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_11: *mut el = 0 as *mut el;
    if !headB.is_null() {} else {
        __assert_fail(
            b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
            b"test70.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1767: {
        if !headB.is_null() {} else {
            __assert_fail(
                b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1717: {
        if !(&mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[18]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
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
            b"test70.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1662: {
        if !(&mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[19]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test70.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    els[19 as libc::c_int as usize].next = els[18 as libc::c_int as usize].next;
    if headB == &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el {
        headB = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
    } else {
        _tmp_11 = headB;
        while !((*_tmp_11).next).is_null()
            && (*_tmp_11).next
                != &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el
        {
            _tmp_11 = (*_tmp_11).next;
        }
        if !((*_tmp_11).next).is_null() {
            (*_tmp_11)
                .next = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize)
                as *mut el;
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
