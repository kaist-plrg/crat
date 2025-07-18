use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PAGE_T {
    pub buf: *mut libc::c_char,
    pub len: size_t,
    pub size: size_t,
}
pub type PAGE = *mut PAGE_T;
pub static mut PAGESIZE: size_t = ::std::mem::size_of::<PAGE_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_page(mut str: *mut libc::c_char) -> PAGE {
    let mut this: PAGE = 0 as *mut PAGE_T;
    this = calloc(1 as libc::c_int as libc::c_ulong, PAGESIZE) as PAGE;
    (*this).len = strlen(str);
    (*this).size = ((*this).len).wrapping_add(24576 as libc::c_int as libc::c_ulong);
    (*this)
        .buf = calloc(1 as libc::c_int as libc::c_ulong, (*this).size)
        as *mut libc::c_char;
    memcpy((*this).buf as *mut libc::c_void, str as *const libc::c_void, (*this).len);
    return this;
}
pub unsafe extern "C" fn page_destroy(mut this: PAGE) -> PAGE {
    if !this.is_null() {
        (*this).len = 0 as libc::c_int as size_t;
        (*this).size = 0 as libc::c_int as size_t;
        free((*this).buf as *mut libc::c_void);
        free(this as *mut libc::c_void);
    }
    return this;
}
pub unsafe extern "C" fn page_length(mut this: PAGE) -> size_t {
    return (*this).len;
}
pub unsafe extern "C" fn page_size(mut this: PAGE) -> size_t {
    return (*this).size;
}
pub unsafe extern "C" fn page_concat(
    mut this: PAGE,
    mut str: *const libc::c_char,
    len: libc::c_int,
) {
    if this.is_null() || str.is_null() || strlen(str) < 1 as libc::c_int as libc::c_ulong
        || len < 0 as libc::c_int
    {
        return;
    }
    if ((*this).len).wrapping_add(len as libc::c_ulong) > (*this).size {
        __expand(this, len + 1 as libc::c_int);
    }
    memcpy(
        ((*this).buf).offset((*this).len as isize) as *mut libc::c_void,
        str as *const libc::c_void,
        len as libc::c_ulong,
    );
    (*this)
        .len = ((*this).len as libc::c_ulong).wrapping_add(len as libc::c_ulong)
        as size_t as size_t;
    *((*this).buf)
        .offset(
            ((*this).len).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn page_clear(mut this: PAGE) {
    if this.is_null() {
        return;
    }
    (*this).len = 0 as libc::c_int as size_t;
    memset((*this).buf as *mut libc::c_void, '\0' as i32, (*this).size);
}
pub unsafe extern "C" fn page_value(mut this: PAGE) -> *mut libc::c_char {
    return (*this).buf;
}
pub unsafe extern "C" fn __expand(mut this: PAGE, len: libc::c_int) {
    if this.is_null() || len < 0 as libc::c_int {
        return;
    }
    (*this)
        .size = ((*this).size as libc::c_ulong).wrapping_add(len as libc::c_ulong)
        as size_t as size_t;
    (*this)
        .buf = realloc((*this).buf as *mut libc::c_void, (*this).size)
        as *mut libc::c_char;
    memset(
        ((*this).buf).offset(((*this).size).wrapping_sub(len as libc::c_ulong) as isize)
            as *mut libc::c_void,
        '\0' as i32,
        len as libc::c_ulong,
    );
}
