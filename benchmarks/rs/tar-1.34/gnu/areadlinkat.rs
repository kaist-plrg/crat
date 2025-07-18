use ::libc;
extern "C" {
    pub type allocator;
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
    fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub unsafe extern "C" fn areadlinkat(
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
) -> *mut libc::c_char {
    return careadlinkat(
        fd,
        filename,
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        0 as *const allocator,
        Some(
            readlinkat
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
