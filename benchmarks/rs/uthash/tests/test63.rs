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
    printf(b"LL macros\n\0" as *const u8 as *const libc::c_char);
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
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_2: *mut el = 0 as *mut el;
    els[3 as libc::c_int as usize].next = 0 as *mut el;
    if !headB.is_null() {
        _tmp_2 = headB;
        while !((*_tmp_2).next).is_null() {
            _tmp_2 = (*_tmp_2).next;
        }
        (*_tmp_2)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_3: *mut el = 0 as *mut el;
    els[4 as libc::c_int as usize].next = 0 as *mut el;
    if !headB.is_null() {
        _tmp_3 = headB;
        while !((*_tmp_3).next).is_null() {
            _tmp_3 = (*_tmp_3).next;
        }
        (*_tmp_3)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_4: *mut el = 0 as *mut el;
    els[5 as libc::c_int as usize].next = 0 as *mut el;
    if !headB.is_null() {
        _tmp_4 = headB;
        while !((*_tmp_4).next).is_null() {
            _tmp_4 = (*_tmp_4).next;
        }
        (*_tmp_4)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_5: *mut el = 0 as *mut el;
    if !headA.is_null() {
        _tmp_5 = headA;
        while !((*_tmp_5).next).is_null() {
            _tmp_5 = (*_tmp_5).next;
        }
        (*_tmp_5).next = headB;
    } else {
        headA = headB;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    headA = 0 as *mut el;
    let mut _tmp_6: *mut el = 0 as *mut el;
    if !headA.is_null() {
        _tmp_6 = headA;
        while !((*_tmp_6).next).is_null() {
            _tmp_6 = (*_tmp_6).next;
        }
        (*_tmp_6).next = headB;
    } else {
        headA = headB;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    headB = 0 as *mut el;
    let mut _tmp_7: *mut el = 0 as *mut el;
    if !headA.is_null() {
        _tmp_7 = headA;
        while !((*_tmp_7).next).is_null() {
            _tmp_7 = (*_tmp_7).next;
        }
        (*_tmp_7).next = headB;
    } else {
        headA = headB;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    headA = 0 as *mut el;
    headB = 0 as *mut el;
    let mut _tmp_8: *mut el = 0 as *mut el;
    els[0 as libc::c_int as usize].next = 0 as *mut el;
    if !headA.is_null() {
        _tmp_8 = headA;
        while !((*_tmp_8).next).is_null() {
            _tmp_8 = (*_tmp_8).next;
        }
        (*_tmp_8)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_9: *mut el = 0 as *mut el;
    els[1 as libc::c_int as usize].next = 0 as *mut el;
    if !headB.is_null() {
        _tmp_9 = headB;
        while !((*_tmp_9).next).is_null() {
            _tmp_9 = (*_tmp_9).next;
        }
        (*_tmp_9)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_10: *mut el = 0 as *mut el;
    if !headA.is_null() {
        _tmp_10 = headA;
        while !((*_tmp_10).next).is_null() {
            _tmp_10 = (*_tmp_10).next;
        }
        (*_tmp_10).next = headB;
    } else {
        headA = headB;
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
