use ::libc;
extern "C" {
    pub type allocator;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn careadlinkat(
        fd: libc::c_int,
        filename: *const libc::c_char,
        buffer: *mut libc::c_char,
        buffer_size: size_t,
        alloc: *const allocator,
        preadlinkat: Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_char,
                *mut libc::c_char,
                size_t,
            ) -> ssize_t,
        >,
    ) -> *mut libc::c_char;
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
unsafe extern "C" fn careadlinkatcwd(
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> ssize_t {
    if fd != -(100 as libc::c_int) {
        abort();
    }
    return readlink(filename, buffer, buffer_size);
}
pub unsafe extern "C" fn areadlink(
    mut filename: *const libc::c_char,
) -> *mut libc::c_char {
    return careadlinkat(
        -(100 as libc::c_int),
        filename,
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        0 as *const allocator,
        Some(
            careadlinkatcwd
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
