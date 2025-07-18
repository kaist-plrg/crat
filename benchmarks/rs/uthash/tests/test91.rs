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
    pub score: libc::c_int,
    pub next: *mut el,
    pub prev: *mut el,
}
unsafe extern "C" fn order_desc(mut a: *mut el, mut b: *mut el) -> libc::c_int {
    return if (*a).score > (*b).score {
        -(1 as libc::c_int)
    } else {
        ((*a).score < (*b).score) as libc::c_int
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut head: *mut el = 0 as *mut el;
    let mut els: [el; 15] = [el {
        id: 0,
        score: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 15];
    let mut e: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        els[i as usize].score = i % 7 as libc::c_int;
        let mut _tmp: *mut el = 0 as *mut el;
        if !head.is_null() {
            if head.is_null()
                || order_desc(head, &mut *els.as_mut_ptr().offset(i as isize))
                    >= 0 as libc::c_int
            {
                _tmp = 0 as *mut el;
            } else {
                _tmp = head;
                while !((*_tmp).next).is_null() {
                    if order_desc(
                        (*_tmp).next,
                        &mut *els.as_mut_ptr().offset(i as isize),
                    ) >= 0 as libc::c_int
                    {
                        break;
                    }
                    _tmp = (*_tmp).next;
                }
            }
            if !_tmp.is_null() {
                if !head.is_null() {} else {
                    __assert_fail(
                        b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test91.c\0" as *const u8 as *const libc::c_char,
                        23 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
                'c_1800: {
                    if !head.is_null() {} else {
                        __assert_fail(
                            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                            b"test91.c\0" as *const u8 as *const libc::c_char,
                            23 as libc::c_int as libc::c_uint,
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
                        b"test91.c\0" as *const u8 as *const libc::c_char,
                        23 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
                'c_1748: {
                    if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
                    {} else {
                        __assert_fail(
                            b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                            b"test91.c\0" as *const u8 as *const libc::c_char,
                            23 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 11],
                                &[libc::c_char; 11],
                            >(b"int main()\0"))
                                .as_ptr(),
                        );
                    }
                };
                els[i as usize].next = (*_tmp).next;
                (*_tmp).next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            } else {
                els[i as usize].next = head;
                head = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            }
        } else {
            head = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            (*head).next = 0 as *mut el;
        }
        i += 1;
        i;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"DL_INSERT_INORDER\n\0" as *const u8 as *const libc::c_char);
    head = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        let mut _tmp_0: *mut el = 0 as *mut el;
        if !head.is_null() {
            if head.is_null()
                || order_desc(head, &mut *els.as_mut_ptr().offset(i as isize))
                    >= 0 as libc::c_int
            {
                _tmp_0 = 0 as *mut el;
            } else {
                _tmp_0 = head;
                while !((*_tmp_0).next).is_null() {
                    if order_desc(
                        (*_tmp_0).next,
                        &mut *els.as_mut_ptr().offset(i as isize),
                    ) >= 0 as libc::c_int
                    {
                        break;
                    }
                    _tmp_0 = (*_tmp_0).next;
                }
            }
            if !_tmp_0.is_null() {
                if !head.is_null() {} else {
                    __assert_fail(
                        b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test91.c\0" as *const u8 as *const libc::c_char,
                        33 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
                'c_1465: {
                    if !head.is_null() {} else {
                        __assert_fail(
                            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                            b"test91.c\0" as *const u8 as *const libc::c_char,
                            33 as libc::c_int as libc::c_uint,
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
                        b"test91.c\0" as *const u8 as *const libc::c_char,
                        33 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
                'c_1413: {
                    if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
                    {} else {
                        __assert_fail(
                            b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                            b"test91.c\0" as *const u8 as *const libc::c_char,
                            33 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 11],
                                &[libc::c_char; 11],
                            >(b"int main()\0"))
                                .as_ptr(),
                        );
                    }
                };
                els[i as usize].next = (*_tmp_0).next;
                els[i as usize].prev = _tmp_0;
                (*_tmp_0).next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                if !(els[i as usize].next).is_null() {
                    (*els[i as usize].next)
                        .prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                } else {
                    (*head).prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                }
            } else {
                els[i as usize].next = head;
                if !head.is_null() {
                    els[i as usize].prev = (*head).prev;
                    (*head).prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                } else {
                    els[i as usize]
                        .prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                }
                head = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            }
        } else {
            head = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            (*head).prev = head;
            (*head).next = 0 as *mut el;
        }
        i += 1;
        i;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"CDL_INSERT_INORDER\n\0" as *const u8 as *const libc::c_char);
    head = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        let mut _tmp_1: *mut el = 0 as *mut el;
        if !head.is_null() {
            if head.is_null()
                || order_desc(head, &mut *els.as_mut_ptr().offset(i as isize))
                    >= 0 as libc::c_int
            {
                _tmp_1 = 0 as *mut el;
            } else {
                _tmp_1 = head;
                while (*_tmp_1).next != head {
                    if order_desc(
                        (*_tmp_1).next,
                        &mut *els.as_mut_ptr().offset(i as isize),
                    ) >= 0 as libc::c_int
                    {
                        break;
                    }
                    _tmp_1 = (*_tmp_1).next;
                }
            }
            if !_tmp_1.is_null() {
                if !head.is_null() {} else {
                    __assert_fail(
                        b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test91.c\0" as *const u8 as *const libc::c_char,
                        43 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
                'c_1015: {
                    if !head.is_null() {} else {
                        __assert_fail(
                            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                            b"test91.c\0" as *const u8 as *const libc::c_char,
                            43 as libc::c_int as libc::c_uint,
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
                        b"test91.c\0" as *const u8 as *const libc::c_char,
                        43 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
                'c_958: {
                    if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
                    {} else {
                        __assert_fail(
                            b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                            b"test91.c\0" as *const u8 as *const libc::c_char,
                            43 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 11],
                                &[libc::c_char; 11],
                            >(b"int main()\0"))
                                .as_ptr(),
                        );
                    }
                };
                els[i as usize].next = (*_tmp_1).next;
                els[i as usize].prev = _tmp_1;
                (*_tmp_1).next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                (*els[i as usize].next)
                    .prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            } else {
                if !head.is_null() {
                    els[i as usize].prev = (*head).prev;
                    els[i as usize].next = head;
                    (*head).prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                    (*els[i as usize].prev)
                        .next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                } else {
                    els[i as usize]
                        .prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                    els[i as usize]
                        .next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
                }
                head = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            }
        } else {
            head = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            (*head).next = head;
            (*head).prev = head;
        }
        i += 1;
        i;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).prev == head { 0 as *mut el } else { (*e).prev };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
