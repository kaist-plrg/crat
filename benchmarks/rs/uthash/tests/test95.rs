use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct el {
    pub id: libc::c_int,
    pub score: libc::c_int,
    pub next: *mut el,
    pub next2: *mut el,
}
unsafe fn main_0() -> libc::c_int {
    let mut alpha: el = {
        let mut init = el {
            id: 1 as libc::c_int,
            score: 100 as libc::c_int,
            next: 0 as *mut el,
            next2: 0 as *mut el,
        };
        init
    };
    let mut beta: el = {
        let mut init = el {
            id: 2 as libc::c_int,
            score: 100 as libc::c_int,
            next: 0 as *mut el,
            next2: 0 as *mut el,
        };
        init
    };
    let mut gamma: el = {
        let mut init = el {
            id: 3 as libc::c_int,
            score: 100 as libc::c_int,
            next: 0 as *mut el,
            next2: 0 as *mut el,
        };
        init
    };
    let mut delta: el = {
        let mut init = el {
            id: 4 as libc::c_int,
            score: 100 as libc::c_int,
            next: 0 as *mut el,
            next2: 0 as *mut el,
        };
        init
    };
    let mut stack1: *mut el = 0 as *mut el;
    let mut stack2: *mut el = 0 as *mut el;
    let mut dummy: *mut el = 0 as *mut el;
    let mut size1: libc::c_int = 0;
    let mut size2: size_t = 0;
    size1 = 0 as libc::c_int;
    dummy = stack1;
    while !dummy.is_null() {
        size1 += 1;
        size1;
        dummy = (*dummy).next;
    }
    if size1 == 0 as libc::c_int {} else {
        __assert_fail(
            b"size1 == 0\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            24 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1680: {
        if size1 == 0 as libc::c_int {} else {
            __assert_fail(
                b"size1 == 0\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                24 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    size2 = 0 as libc::c_int as size_t;
    dummy = stack2;
    while !dummy.is_null() {
        size2 = size2.wrapping_add(1);
        size2;
        dummy = (*dummy).next;
    }
    if size2 == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"size2 == 0\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1611: {
        if size2 == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"size2 == 0\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                25 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if stack1.is_null() {} else {
        __assert_fail(
            b"STACK_EMPTY(stack1)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1573: {
        if stack1.is_null() {} else {
            __assert_fail(
                b"STACK_EMPTY(stack1)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                26 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if stack2.is_null() {} else {
        __assert_fail(
            b"STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1535: {
        if stack2.is_null() {} else {
            __assert_fail(
                b"STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                27 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    alpha.next = stack1;
    stack1 = &mut alpha;
    size1 = 0 as libc::c_int;
    dummy = stack1;
    while !dummy.is_null() {
        size1 += 1;
        size1;
        dummy = (*dummy).next;
    }
    if size1 == 1 as libc::c_int {} else {
        __assert_fail(
            b"size1 == 1\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1451: {
        if size1 == 1 as libc::c_int {} else {
            __assert_fail(
                b"size1 == 1\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    beta.next = stack1;
    stack1 = &mut beta;
    size1 = 0 as libc::c_int;
    dummy = stack1;
    while !dummy.is_null() {
        size1 += 1;
        size1;
        dummy = (*dummy).next;
    }
    if size1 == 2 as libc::c_int {} else {
        __assert_fail(
            b"size1 == 2\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1367: {
        if size1 == 2 as libc::c_int {} else {
            __assert_fail(
                b"size1 == 2\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    gamma.next = stack1;
    stack1 = &mut gamma;
    delta.next = stack1;
    stack1 = &mut delta;
    size1 = 0 as libc::c_int;
    dummy = stack1;
    while !dummy.is_null() {
        size1 += 1;
        size1;
        dummy = (*dummy).next;
    }
    if size1 == 4 as libc::c_int {} else {
        __assert_fail(
            b"size1 == 4\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1265: {
        if size1 == 4 as libc::c_int {} else {
            __assert_fail(
                b"size1 == 4\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                35 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if stack1 == &mut delta as *mut el {} else {
        __assert_fail(
            b"stack1 == &delta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1227: {
        if stack1 == &mut delta as *mut el {} else {
            __assert_fail(
                b"stack1 == &delta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                36 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    alpha.next2 = stack2;
    stack2 = &mut alpha;
    if stack2 == &mut alpha as *mut el {} else {
        __assert_fail(
            b"stack2 == &alpha\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1171: {
        if stack2 == &mut alpha as *mut el {} else {
            __assert_fail(
                b"stack2 == &alpha\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if (alpha.next2).is_null() {} else {
        __assert_fail(
            b"alpha.next2 == NULL\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1127: {
        if (alpha.next2).is_null() {} else {
            __assert_fail(
                b"alpha.next2 == NULL\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    delta.next2 = stack2;
    stack2 = &mut delta;
    if stack2 == &mut delta as *mut el {} else {
        __assert_fail(
            b"stack2 == &delta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1071: {
        if stack2 == &mut delta as *mut el {} else {
            __assert_fail(
                b"stack2 == &delta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                42 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if delta.next2 == &mut alpha as *mut el {} else {
        __assert_fail(
            b"delta.next2 == &alpha\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1030: {
        if delta.next2 == &mut alpha as *mut el {} else {
            __assert_fail(
                b"delta.next2 == &alpha\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    size2 = 0 as libc::c_int as size_t;
    dummy = stack2;
    while !dummy.is_null() {
        size2 = size2.wrapping_add(1);
        size2;
        dummy = (*dummy).next2;
    }
    if size2 == 2 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"size2 == 2\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_959: {
        if size2 == 2 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"size2 == 2\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !stack2.is_null() {} else {
        __assert_fail(
            b"!STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_919: {
        if !stack2.is_null() {} else {
            __assert_fail(
                b"!STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if stack2 == &mut delta as *mut el {} else {
        __assert_fail(
            b"stack2 == &delta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_881: {
        if stack2 == &mut delta as *mut el {} else {
            __assert_fail(
                b"stack2 == &delta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !stack1.is_null() {} else {
        __assert_fail(
            b"!STACK_EMPTY(stack1)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_841: {
        if !stack1.is_null() {} else {
            __assert_fail(
                b"!STACK_EMPTY(stack1)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !stack2.is_null() {} else {
        __assert_fail(
            b"!STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_800: {
        if !stack2.is_null() {} else {
            __assert_fail(
                b"!STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    dummy = stack1;
    stack1 = (*stack1).next;
    if stack1 == &mut gamma as *mut el {} else {
        __assert_fail(
            b"stack1 == &gamma\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_744: {
        if stack1 == &mut gamma as *mut el {} else {
            __assert_fail(
                b"stack1 == &gamma\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if dummy == &mut delta as *mut el {} else {
        __assert_fail(
            b"dummy == &delta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_706: {
        if dummy == &mut delta as *mut el {} else {
            __assert_fail(
                b"dummy == &delta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    dummy = stack1;
    stack1 = (*stack1).next;
    if stack1 == &mut beta as *mut el {} else {
        __assert_fail(
            b"stack1 == &beta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_650: {
        if stack1 == &mut beta as *mut el {} else {
            __assert_fail(
                b"stack1 == &beta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if dummy == &mut gamma as *mut el {} else {
        __assert_fail(
            b"dummy == &gamma\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_599: {
        if dummy == &mut gamma as *mut el {} else {
            __assert_fail(
                b"dummy == &gamma\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    dummy = stack1;
    stack1 = (*stack1).next;
    if stack1 == &mut alpha as *mut el {} else {
        __assert_fail(
            b"stack1 == &alpha\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_542: {
        if stack1 == &mut alpha as *mut el {} else {
            __assert_fail(
                b"stack1 == &alpha\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if dummy == &mut beta as *mut el {} else {
        __assert_fail(
            b"dummy == &beta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_491: {
        if dummy == &mut beta as *mut el {} else {
            __assert_fail(
                b"dummy == &beta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    size1 = 0 as libc::c_int;
    dummy = stack1;
    while !dummy.is_null() {
        size1 += 1;
        size1;
        dummy = (*dummy).next;
    }
    if size1 == 1 as libc::c_int {} else {
        __assert_fail(
            b"size1 == 1\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_424: {
        if size1 == 1 as libc::c_int {} else {
            __assert_fail(
                b"size1 == 1\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    dummy = stack1;
    stack1 = (*stack1).next;
    if stack1.is_null() {} else {
        __assert_fail(
            b"stack1 == NULL\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_363: {
        if stack1.is_null() {} else {
            __assert_fail(
                b"stack1 == NULL\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if dummy == &mut alpha as *mut el {} else {
        __assert_fail(
            b"dummy == &alpha\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_311: {
        if dummy == &mut alpha as *mut el {} else {
            __assert_fail(
                b"dummy == &alpha\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if stack2 == &mut delta as *mut el {} else {
        __assert_fail(
            b"STACK_TOP(stack2) == &delta\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_257: {
        if stack2 == &mut delta as *mut el {} else {
            __assert_fail(
                b"STACK_TOP(stack2) == &delta\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    while !stack2.is_null() {
        dummy = stack2;
        stack2 = (*stack2).next2;
    }
    if stack1.is_null() {} else {
        __assert_fail(
            b"STACK_EMPTY(stack1)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_186: {
        if stack1.is_null() {} else {
            __assert_fail(
                b"STACK_EMPTY(stack1)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if stack2.is_null() {} else {
        __assert_fail(
            b"STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
            b"test95.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_135: {
        if stack2.is_null() {} else {
            __assert_fail(
                b"STACK_EMPTY(stack2)\0" as *const u8 as *const libc::c_char,
                b"test95.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
