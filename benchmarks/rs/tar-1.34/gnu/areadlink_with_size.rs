use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub const stackbuf_size: C2RustUnnamed = 128;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub unsafe extern "C" fn areadlink_with_size(
    mut file: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut symlink_max: size_t = 1024 as libc::c_int as size_t;
    let mut INITIAL_LIMIT_BOUND: size_t = (8 as libc::c_int * 1024 as libc::c_int)
        as size_t;
    let mut initial_limit: size_t = if symlink_max < INITIAL_LIMIT_BOUND {
        symlink_max.wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        INITIAL_LIMIT_BOUND
    };
    let mut buf_size: size_t = if size == 0 as libc::c_int as libc::c_ulong {
        stackbuf_size as libc::c_int as libc::c_ulong
    } else if size < initial_limit {
        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        initial_limit
    };
    loop {
        let mut r: ssize_t = 0;
        let mut link_length: size_t = 0;
        let mut stackbuf: [libc::c_char; 128] = [0; 128];
        let mut buf: *mut libc::c_char = stackbuf.as_mut_ptr();
        let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
        if !(size == 0 as libc::c_int as libc::c_ulong
            && buf_size == stackbuf_size as libc::c_int as libc::c_ulong)
        {
            buffer = malloc(buf_size) as *mut libc::c_char;
            buf = buffer;
            if buffer.is_null() {
                *__errno_location() = 12 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
        }
        r = readlink(file, buf, buf_size);
        link_length = r as size_t;
        if r < 0 as libc::c_int as libc::c_long {
            let mut saved_errno: libc::c_int = *__errno_location();
            rpl_free(buffer as *mut libc::c_void);
            *__errno_location() = saved_errno;
            return 0 as *mut libc::c_char;
        }
        if link_length < buf_size {
            *buf.offset(link_length as isize) = 0 as libc::c_int as libc::c_char;
            if buffer.is_null() {
                buffer = malloc(
                    link_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if !buffer.is_null() {
                    return memcpy(
                        buffer as *mut libc::c_void,
                        buf as *const libc::c_void,
                        link_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                }
            } else if link_length.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < buf_size
            {
                let mut shrinked_buffer: *mut libc::c_char = realloc(
                    buffer as *mut libc::c_void,
                    link_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if !shrinked_buffer.is_null() {
                    buffer = shrinked_buffer;
                }
            }
            return buffer;
        }
        rpl_free(buffer as *mut libc::c_void);
        if buf_size
            <= (if (18446744073709551615 as libc::c_ulong)
                < 9223372036854775807 as libc::c_long as libc::c_ulong
            {
                18446744073709551615 as libc::c_ulong
            } else {
                9223372036854775807 as libc::c_long as libc::c_ulong
            })
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            buf_size = (buf_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else if buf_size
            < (if (18446744073709551615 as libc::c_ulong)
                < 9223372036854775807 as libc::c_long as libc::c_ulong
            {
                18446744073709551615 as libc::c_ulong
            } else {
                9223372036854775807 as libc::c_long as libc::c_ulong
            })
        {
            buf_size = if (18446744073709551615 as libc::c_ulong)
                < 9223372036854775807 as libc::c_long as libc::c_ulong
            {
                18446744073709551615 as libc::c_ulong
            } else {
                9223372036854775807 as libc::c_long as libc::c_ulong
            };
        } else {
            *__errno_location() = 12 as libc::c_int;
            return 0 as *mut libc::c_char;
        }
    };
}
