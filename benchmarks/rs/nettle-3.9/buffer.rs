use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
pub unsafe extern "C" fn nettle_buffer_grow(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
) -> libc::c_int {
    if (*buffer).size <= (*buffer).alloc {} else {
        __assert_fail(
            b"buffer->size <= buffer->alloc\0" as *const u8 as *const libc::c_char,
            b"buffer.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int nettle_buffer_grow(struct nettle_buffer *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1807: {
        if (*buffer).size <= (*buffer).alloc {} else {
            __assert_fail(
                b"buffer->size <= buffer->alloc\0" as *const u8 as *const libc::c_char,
                b"buffer.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int nettle_buffer_grow(struct nettle_buffer *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*buffer).size).wrapping_add(length) > (*buffer).alloc {
        let mut alloc: size_t = 0;
        let mut p: *mut uint8_t = 0 as *mut uint8_t;
        if ((*buffer).realloc).is_none() {
            return 0 as libc::c_int;
        }
        alloc = ((*buffer).alloc)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(length)
            .wrapping_add(100 as libc::c_int as libc::c_ulong);
        p = ((*buffer).realloc)
            .unwrap()(
            (*buffer).realloc_ctx,
            (*buffer).contents as *mut libc::c_void,
            alloc,
        ) as *mut uint8_t;
        if p.is_null() {
            return 0 as libc::c_int;
        }
        (*buffer).contents = p;
        (*buffer).alloc = alloc;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_buffer_init_realloc(
    mut buffer: *mut nettle_buffer,
    mut realloc_ctx: *mut libc::c_void,
    mut realloc: Option::<nettle_realloc_func>,
) {
    (*buffer).contents = 0 as *mut uint8_t;
    (*buffer).alloc = 0 as libc::c_int as size_t;
    (*buffer).realloc = realloc;
    (*buffer).realloc_ctx = realloc_ctx;
    (*buffer).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn nettle_buffer_init_size(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut space: *mut uint8_t,
) {
    (*buffer).contents = space;
    (*buffer).alloc = length;
    (*buffer).realloc = None;
    (*buffer).realloc_ctx = 0 as *mut libc::c_void;
    (*buffer).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn nettle_buffer_clear(mut buffer: *mut nettle_buffer) {
    if ((*buffer).realloc).is_some() {
        ((*buffer).realloc)
            .unwrap()(
            (*buffer).realloc_ctx,
            (*buffer).contents as *mut libc::c_void,
            0 as libc::c_int as size_t,
        );
    }
    (*buffer).contents = 0 as *mut uint8_t;
    (*buffer).alloc = 0 as libc::c_int as size_t;
    (*buffer).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn nettle_buffer_reset(mut buffer: *mut nettle_buffer) {
    (*buffer).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn nettle_buffer_space(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
) -> *mut uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    if nettle_buffer_grow(buffer, length) == 0 {
        return 0 as *mut uint8_t;
    }
    p = ((*buffer).contents).offset((*buffer).size as isize);
    (*buffer)
        .size = ((*buffer).size as libc::c_ulong).wrapping_add(length) as size_t
        as size_t;
    return p;
}
pub unsafe extern "C" fn nettle_buffer_write(
    mut buffer: *mut nettle_buffer,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut p: *mut uint8_t = nettle_buffer_space(buffer, length);
    if !p.is_null() {
        memcpy(p as *mut libc::c_void, data as *const libc::c_void, length);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn nettle_buffer_copy(
    mut dst: *mut nettle_buffer,
    mut src: *const nettle_buffer,
) -> libc::c_int {
    return nettle_buffer_write(dst, (*src).size, (*src).contents);
}
