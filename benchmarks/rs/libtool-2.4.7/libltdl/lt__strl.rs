use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn lt_strlcat(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    dstsize: size_t,
) -> size_t {
    let mut length: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    if !dst.is_null() {} else {
        __assert_fail(
            b"dst != NULL\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"size_t lt_strlcat(char *, const char *, const size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1652: {
        if !dst.is_null() {} else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
                58 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"size_t lt_strlcat(char *, const char *, const size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if !src.is_null() {} else {
        __assert_fail(
            b"src != (const char *) NULL\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"size_t lt_strlcat(char *, const char *, const size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1609: {
        if !src.is_null() {} else {
            __assert_fail(
                b"src != (const char *) NULL\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
                59 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"size_t lt_strlcat(char *, const char *, const size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if dstsize >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"dstsize >= 1\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"size_t lt_strlcat(char *, const char *, const size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1566: {
        if dstsize >= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"dstsize >= 1\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"size_t lt_strlcat(char *, const char *, const size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    length = strlen(dst);
    p = dst.offset(length as isize);
    q = src;
    while *q as libc::c_int != 0 as libc::c_int
        && length < dstsize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        *p = *q;
        length = length.wrapping_add(1);
        length;
        p = p.offset(1);
        p;
        q = q.offset(1);
        q;
    }
    *dst.offset(length as isize) = '\0' as i32 as libc::c_char;
    loop {
        let fresh0 = q;
        q = q.offset(1);
        if !(*fresh0 != 0) {
            break;
        }
        length = length.wrapping_add(1);
        length;
    }
    return length;
}
pub unsafe extern "C" fn lt_strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    dstsize: size_t,
) -> size_t {
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    if !dst.is_null() {} else {
        __assert_fail(
            b"dst != NULL\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"size_t lt_strlcpy(char *, const char *, const size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1862: {
        if !dst.is_null() {} else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"size_t lt_strlcpy(char *, const char *, const size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if !src.is_null() {} else {
        __assert_fail(
            b"src != (const char *) NULL\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"size_t lt_strlcpy(char *, const char *, const size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1820: {
        if !src.is_null() {} else {
            __assert_fail(
                b"src != (const char *) NULL\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"size_t lt_strlcpy(char *, const char *, const size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if dstsize >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"dstsize >= 1\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"size_t lt_strlcpy(char *, const char *, const size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1782: {
        if dstsize >= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"dstsize >= 1\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt__strl.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"size_t lt_strlcpy(char *, const char *, const size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    p = dst;
    q = src;
    length = 0 as libc::c_int as size_t;
    while *q as libc::c_int != 0 as libc::c_int
        && length < dstsize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        *p = *q;
        length = length.wrapping_add(1);
        length;
        p = p.offset(1);
        p;
        q = q.offset(1);
        q;
    }
    *dst.offset(length as isize) = '\0' as i32 as libc::c_char;
    loop {
        let fresh1 = q;
        q = q.offset(1);
        if !(*fresh1 != 0) {
            break;
        }
        length = length.wrapping_add(1);
        length;
    }
    return length;
}
