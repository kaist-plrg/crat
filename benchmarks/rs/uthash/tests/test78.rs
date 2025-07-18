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
    pub Next: *mut el,
    pub Prev: *mut el,
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut els: [el; 10] = [el {
        id: 0,
        Next: 0 as *mut el,
        Prev: 0 as *mut el,
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
        els[0 as libc::c_int as usize].Prev = (*head).Prev;
        els[0 as libc::c_int as usize].Next = head;
        (*head)
            .Prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*els[0 as libc::c_int as usize].Prev)
            .Next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        els[0 as libc::c_int as usize]
            .Prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize]
            .Next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    if !head.is_null() {
        els[1 as libc::c_int as usize].Prev = (*head).Prev;
        els[1 as libc::c_int as usize].Next = head;
        (*head)
            .Prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*els[1 as libc::c_int as usize].Prev)
            .Next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        els[1 as libc::c_int as usize]
            .Prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize]
            .Next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    if !head.is_null() {
        els[2 as libc::c_int as usize].Prev = (*head).Prev;
        els[2 as libc::c_int as usize].Next = head;
        (*head)
            .Prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*els[2 as libc::c_int as usize].Prev)
            .Next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        els[2 as libc::c_int as usize]
            .Prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[2 as libc::c_int as usize]
            .Next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).Next == head { 0 as *mut el } else { (*e).Next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"advancing head pointer\n\0" as *const u8 as *const libc::c_char);
    head = (*head).Next;
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).Next == head { 0 as *mut el } else { (*e).Next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    e = head;
    while !e.is_null() && i < 10 as libc::c_int {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        i += 1;
        i;
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    e = head;
    while !e.is_null() && i < 10 as libc::c_int {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        i += 1;
        i;
        e = (*e).Prev;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting b\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
        && (*head).Next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[1 as libc::c_int as usize].Next)
            .Prev = els[1 as libc::c_int as usize].Prev;
        (*els[1 as libc::c_int as usize].Prev)
            .Next = els[1 as libc::c_int as usize].Next;
        if &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el == head {
            head = els[1 as libc::c_int as usize].Next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).Next == head { 0 as *mut el } else { (*e).Next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (a)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
        && (*head).Next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[0 as libc::c_int as usize].Next)
            .Prev = els[0 as libc::c_int as usize].Prev;
        (*els[0 as libc::c_int as usize].Prev)
            .Next = els[0 as libc::c_int as usize].Next;
        if &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el == head {
            head = els[0 as libc::c_int as usize].Next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).Next == head { 0 as *mut el } else { (*e).Next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (c)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
        && (*head).Next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[2 as libc::c_int as usize].Next)
            .Prev = els[2 as libc::c_int as usize].Prev;
        (*els[2 as libc::c_int as usize].Prev)
            .Next = els[2 as libc::c_int as usize].Next;
        if &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el == head {
            head = els[2 as libc::c_int as usize].Next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).Next == head { 0 as *mut el } else { (*e).Next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"DL macros\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {
        els[0 as libc::c_int as usize].Prev = (*head).Prev;
        (*(*head).Prev)
            .Next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*head)
            .Prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize].Next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*head).Prev = head;
        (*head).Next = 0 as *mut el;
    }
    if !head.is_null() {
        els[1 as libc::c_int as usize].Prev = (*head).Prev;
        (*(*head).Prev)
            .Next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*head)
            .Prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize].Next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*head).Prev = head;
        (*head).Next = 0 as *mut el;
    }
    if !head.is_null() {
        els[2 as libc::c_int as usize].Prev = (*head).Prev;
        (*(*head).Prev)
            .Next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*head)
            .Prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[2 as libc::c_int as usize].Next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*head).Prev = head;
        (*head).Next = 0 as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting tail c\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test78.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2000: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test78.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[2 as libc::c_int as usize].Prev).is_null() {} else {
        __assert_fail(
            b"(&els[2])->Prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test78.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1946: {
        if !(els[2 as libc::c_int as usize].Prev).is_null() {} else {
            __assert_fail(
                b"(&els[2])->Prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test78.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[2 as libc::c_int as usize].Prev
        == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el == head
    {
        (*els[2 as libc::c_int as usize].Next)
            .Prev = els[2 as libc::c_int as usize].Prev;
        head = els[2 as libc::c_int as usize].Next;
    } else {
        (*els[2 as libc::c_int as usize].Prev)
            .Next = els[2 as libc::c_int as usize].Next;
        if !(els[2 as libc::c_int as usize].Next).is_null() {
            (*els[2 as libc::c_int as usize].Next)
                .Prev = els[2 as libc::c_int as usize].Prev;
        } else {
            (*head).Prev = els[2 as libc::c_int as usize].Prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting head a\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test78.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1728: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test78.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[0 as libc::c_int as usize].Prev).is_null() {} else {
        __assert_fail(
            b"(&els[0])->Prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test78.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1674: {
        if !(els[0 as libc::c_int as usize].Prev).is_null() {} else {
            __assert_fail(
                b"(&els[0])->Prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test78.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[0 as libc::c_int as usize].Prev
        == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el == head
    {
        (*els[0 as libc::c_int as usize].Next)
            .Prev = els[0 as libc::c_int as usize].Prev;
        head = els[0 as libc::c_int as usize].Next;
    } else {
        (*els[0 as libc::c_int as usize].Prev)
            .Next = els[0 as libc::c_int as usize].Next;
        if !(els[0 as libc::c_int as usize].Next).is_null() {
            (*els[0 as libc::c_int as usize].Next)
                .Prev = els[0 as libc::c_int as usize].Prev;
        } else {
            (*head).Prev = els[0 as libc::c_int as usize].Prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting head b\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test78.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1455: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test78.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[1 as libc::c_int as usize].Prev).is_null() {} else {
        __assert_fail(
            b"(&els[1])->Prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test78.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1397: {
        if !(els[1 as libc::c_int as usize].Prev).is_null() {} else {
            __assert_fail(
                b"(&els[1])->Prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test78.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[1 as libc::c_int as usize].Prev
        == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el == head
    {
        (*els[1 as libc::c_int as usize].Next)
            .Prev = els[1 as libc::c_int as usize].Prev;
        head = els[1 as libc::c_int as usize].Next;
    } else {
        (*els[1 as libc::c_int as usize].Prev)
            .Next = els[1 as libc::c_int as usize].Next;
        if !(els[1 as libc::c_int as usize].Next).is_null() {
            (*els[1 as libc::c_int as usize].Next)
                .Prev = els[1 as libc::c_int as usize].Prev;
        } else {
            (*head).Prev = els[1 as libc::c_int as usize].Prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"LL macros\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp: *mut el = 0 as *mut el;
    els[0 as libc::c_int as usize].Next = 0 as *mut el;
    if !head.is_null() {
        _tmp = head;
        while !((*_tmp).Next).is_null() {
            _tmp = (*_tmp).Next;
        }
        (*_tmp)
            .Next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_0: *mut el = 0 as *mut el;
    els[1 as libc::c_int as usize].Next = 0 as *mut el;
    if !head.is_null() {
        _tmp_0 = head;
        while !((*_tmp_0).Next).is_null() {
            _tmp_0 = (*_tmp_0).Next;
        }
        (*_tmp_0)
            .Next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_1: *mut el = 0 as *mut el;
    els[2 as libc::c_int as usize].Next = 0 as *mut el;
    if !head.is_null() {
        _tmp_1 = head;
        while !((*_tmp_1).Next).is_null() {
            _tmp_1 = (*_tmp_1).Next;
        }
        (*_tmp_1)
            .Next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting tail c\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_2: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
        head = (*head).Next;
    } else {
        _tmp_2 = head;
        while !((*_tmp_2).Next).is_null()
            && (*_tmp_2).Next
                != &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
        {
            _tmp_2 = (*_tmp_2).Next;
        }
        if !((*_tmp_2).Next).is_null() {
            (*_tmp_2).Next = els[2 as libc::c_int as usize].Next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting head a\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_3: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
        head = (*head).Next;
    } else {
        _tmp_3 = head;
        while !((*_tmp_3).Next).is_null()
            && (*_tmp_3).Next
                != &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
        {
            _tmp_3 = (*_tmp_3).Next;
        }
        if !((*_tmp_3).Next).is_null() {
            (*_tmp_3).Next = els[0 as libc::c_int as usize].Next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting head b\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_4: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
        head = (*head).Next;
    } else {
        _tmp_4 = head;
        while !((*_tmp_4).Next).is_null()
            && (*_tmp_4).Next
                != &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
        {
            _tmp_4 = (*_tmp_4).Next;
        }
        if !((*_tmp_4).Next).is_null() {
            (*_tmp_4).Next = els[1 as libc::c_int as usize].Next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).Next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
