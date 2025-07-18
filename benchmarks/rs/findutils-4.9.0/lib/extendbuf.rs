use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn decide_size(mut current: size_t, mut wanted: size_t) -> size_t {
    let mut newsize: size_t = 0;
    if 0 as libc::c_int as libc::c_ulong == current {
        newsize = 16 as libc::c_int as size_t;
    } else {
        newsize = current;
    }
    while newsize < wanted {
        if (2 as libc::c_int as libc::c_ulong).wrapping_mul(newsize) < newsize {
            return wanted;
        }
        newsize = (newsize as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    return newsize;
}
pub unsafe extern "C" fn extendbuf(
    mut existing: *mut libc::c_void,
    mut wanted: size_t,
    mut allocated: *mut size_t,
) -> *mut libc::c_void {
    let mut saved_errno: libc::c_int = 0;
    let mut newsize: size_t = 0;
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    saved_errno = *__errno_location();
    if wanted > 0 as libc::c_uint as libc::c_ulong {} else {
        __assert_fail(
            b"wanted > 0u\0" as *const u8 as *const libc::c_char,
            b"extendbuf.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1779: {
        if wanted > 0 as libc::c_uint as libc::c_ulong {} else {
            __assert_fail(
                b"wanted > 0u\0" as *const u8 as *const libc::c_char,
                b"extendbuf.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    newsize = decide_size(*allocated, wanted);
    if *allocated == 0 as libc::c_int as libc::c_ulong {
        if existing.is_null() {} else {
            __assert_fail(
                b"NULL == existing\0" as *const u8 as *const libc::c_char,
                b"extendbuf.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_1664: {
            if existing.is_null() {} else {
                __assert_fail(
                    b"NULL == existing\0" as *const u8 as *const libc::c_char,
                    b"extendbuf.c\0" as *const u8 as *const libc::c_char,
                    79 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        *allocated = newsize;
        result = malloc(newsize);
    } else if newsize != *allocated {
        *allocated = newsize;
        result = realloc(existing, newsize);
        if result.is_null() {
            saved_errno = *__errno_location();
        }
    } else {
        result = existing;
    }
    if !result.is_null() {
        *__errno_location() = saved_errno;
    }
    return result;
}
pub unsafe extern "C" fn xextendbuf(
    mut existing: *mut libc::c_void,
    mut wanted: size_t,
    mut allocated: *mut size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = extendbuf(existing, wanted, allocated);
    if p.is_null() {
        rpl_free(existing);
        xalloc_die();
    }
    return p;
}
