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
pub struct bufent {
    pub data: [libc::c_char; 32768],
    pub ptr: *mut libc::c_char,
    pub left: size_t,
    pub next: *mut bufent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuffer {
    pub slots: [bufent; 3],
    pub head: *mut bufent,
    pub tail: *mut bufent,
    pub used: size_t,
}
pub unsafe extern "C" fn ringbuffer_init(mut rb: *mut ringbuffer) {
    (*rb)
        .head = &mut *((*rb).slots).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut bufent;
    (*rb)
        .tail = &mut *((*rb).slots).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut bufent;
    (*rb).used = 0 as libc::c_int as size_t;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < 3 as libc::c_int {
        (*rb)
            .slots[x as usize]
            .next = &mut *((*rb).slots)
            .as_mut_ptr()
            .offset(((x + 1 as libc::c_int) % 3 as libc::c_int) as isize) as *mut bufent;
        x += 1;
        x;
    }
}
pub unsafe extern "C" fn ringbuffer_read_next(
    mut rb: *mut ringbuffer,
    mut length: *mut libc::c_int,
) -> *mut libc::c_char {
    if (*rb).used != 0 {} else {
        __assert_fail(
            b"rb->used\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"char *ringbuffer_read_next(ringbuffer *, int *)\0"))
                .as_ptr(),
        );
    };
    *length = (*(*rb).head).left as libc::c_int;
    return (*(*rb).head).ptr;
}
pub unsafe extern "C" fn ringbuffer_read_skip(
    mut rb: *mut ringbuffer,
    mut length: libc::c_int,
) {
    if (*rb).used != 0 {} else {
        __assert_fail(
            b"rb->used\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void ringbuffer_read_skip(ringbuffer *, int)\0"))
                .as_ptr(),
        );
    };
    (*(*rb).head).ptr = ((*(*rb).head).ptr).offset(length as isize);
    (*(*rb).head)
        .left = ((*(*rb).head).left as libc::c_ulong)
        .wrapping_sub(length as libc::c_ulong) as size_t as size_t;
}
pub unsafe extern "C" fn ringbuffer_read_pop(mut rb: *mut ringbuffer) {
    if (*rb).used != 0 {} else {
        __assert_fail(
            b"rb->used\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void ringbuffer_read_pop(ringbuffer *)\0"))
                .as_ptr(),
        );
    };
    (*rb).head = (*(*rb).head).next;
    (*rb).used = ((*rb).used).wrapping_sub(1);
    (*rb).used;
}
pub unsafe extern "C" fn ringbuffer_write_ptr(
    mut rb: *mut ringbuffer,
) -> *mut libc::c_char {
    if (*rb).used < 3 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"rb->used < RING_SLOTS\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"char *ringbuffer_write_ptr(ringbuffer *)\0"))
                .as_ptr(),
        );
    };
    return ((*(*rb).tail).data).as_mut_ptr();
}
pub unsafe extern "C" fn ringbuffer_write_append(
    mut rb: *mut ringbuffer,
    mut length: libc::c_int,
) {
    if (*rb).used < 3 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"rb->used < RING_SLOTS\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ringbuffer_write_append(ringbuffer *, int)\0"))
                .as_ptr(),
        );
    };
    (*rb).used = ((*rb).used).wrapping_add(1);
    (*rb).used;
    (*(*rb).tail).ptr = ((*(*rb).tail).data).as_mut_ptr();
    (*(*rb).tail).left = length as size_t;
    (*rb).tail = (*(*rb).tail).next;
}
pub unsafe extern "C" fn ringbuffer_size(mut rb: *mut ringbuffer) -> libc::c_int {
    return (*rb).used as libc::c_int;
}
pub unsafe extern "C" fn ringbuffer_capacity(mut rb: *mut ringbuffer) -> libc::c_int {
    return 3 as libc::c_int;
}
pub unsafe extern "C" fn ringbuffer_is_empty(mut rb: *mut ringbuffer) -> libc::c_int {
    return ((*rb).used == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn ringbuffer_is_full(mut rb: *mut ringbuffer) -> libc::c_int {
    return ((*rb).used == 3 as libc::c_int as libc::c_ulong) as libc::c_int;
}
