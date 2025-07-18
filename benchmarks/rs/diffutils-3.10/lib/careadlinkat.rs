use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static stdlib_allocator: allocator;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct allocator {
    pub allocate: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub reallocate: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub die: Option::<unsafe extern "C" fn(size_t) -> ()>,
}
pub type idx_t = ptrdiff_t;
pub type ptrdiff_t = libc::c_long;
pub const STACK_BUF_SIZE: C2RustUnnamed = 1024;
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn readlink_stk(
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut alloc: *const allocator,
    mut preadlinkat: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> ssize_t,
    >,
    mut stack_buf: *mut libc::c_char,
) -> *mut libc::c_char {
    if alloc.is_null() {
        alloc = &stdlib_allocator;
    }
    if buffer.is_null() {
        buffer = stack_buf;
        buffer_size = STACK_BUF_SIZE as libc::c_int as size_t;
    }
    let mut buf: *mut libc::c_char = buffer;
    let mut buf_size_max: idx_t = (if (9223372036854775807 as libc::c_long
        as libc::c_ulong)
        < (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        18446744073709551615 as libc::c_ulong
    }) as idx_t;
    let mut buf_size: idx_t = (if buffer_size < buf_size_max as libc::c_ulong {
        buffer_size
    } else {
        buf_size_max as libc::c_ulong
    }) as idx_t;
    while !buf.is_null() {
        let mut link_length: idx_t = preadlinkat
            .unwrap()(fd, filename, buf, buf_size as size_t);
        if link_length < 0 as libc::c_int as libc::c_long {
            if buf != buffer {
                let mut readlinkat_errno: libc::c_int = *__errno_location();
                ((*alloc).free).unwrap()(buf as *mut libc::c_void);
                *__errno_location() = readlinkat_errno;
            }
            return 0 as *mut libc::c_char;
        }
        let mut link_size: idx_t = link_length;
        if link_size < buf_size {
            let fresh0 = link_size;
            link_size = link_size + 1;
            *buf.offset(fresh0 as isize) = '\0' as i32 as libc::c_char;
            if buf == stack_buf {
                let mut b: *mut libc::c_char = ((*alloc).allocate)
                    .unwrap()(link_size as size_t) as *mut libc::c_char;
                buf_size = link_size;
                if b.is_null() {
                    break;
                }
                return memcpy(
                    b as *mut libc::c_void,
                    buf as *const libc::c_void,
                    link_size as libc::c_ulong,
                ) as *mut libc::c_char;
            } else {
                if link_size < buf_size && buf != buffer
                    && ((*alloc).reallocate).is_some()
                {
                    let mut b_0: *mut libc::c_char = ((*alloc).reallocate)
                        .unwrap()(buf as *mut libc::c_void, link_size as size_t)
                        as *mut libc::c_char;
                    if !b_0.is_null() {
                        return b_0;
                    }
                }
                return buf;
            }
        } else {
            if buf != buffer {
                ((*alloc).free).unwrap()(buf as *mut libc::c_void);
            }
            if buf_size_max / 2 as libc::c_int as libc::c_long <= buf_size {
                *__errno_location() = 36 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            buf_size = 2 as libc::c_int as libc::c_long * buf_size
                + 1 as libc::c_int as libc::c_long;
            buf = ((*alloc).allocate).unwrap()(buf_size as size_t) as *mut libc::c_char;
        }
    }
    if ((*alloc).die).is_some() {
        ((*alloc).die).unwrap()(buf_size as size_t);
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn careadlinkat(
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut alloc: *const allocator,
    mut preadlinkat: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> ssize_t,
    >,
) -> *mut libc::c_char {
    let mut stack_buf: [libc::c_char; 1024] = [0; 1024];
    return readlink_stk(
        fd,
        filename,
        buffer,
        buffer_size,
        alloc,
        preadlinkat,
        stack_buf.as_mut_ptr(),
    );
}
