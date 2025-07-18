use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __libc_dynarray_resize(
        _: *mut dynarray_header,
        size: size_t,
        scratch: *mut libc::c_void,
        element_size: size_t,
    ) -> bool;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
pub unsafe extern "C" fn __libc_dynarray_resize_clear(
    mut list: *mut dynarray_header,
    mut size: size_t,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
) -> bool {
    let mut old_size: size_t = (*list).used;
    if !__libc_dynarray_resize(list, size, scratch, element_size) {
        return 0 as libc::c_int != 0;
    }
    let mut array: *mut libc::c_char = (*list).array as *mut libc::c_char;
    memset(
        array.offset(old_size.wrapping_mul(element_size) as isize) as *mut libc::c_void,
        0 as libc::c_int,
        size.wrapping_sub(old_size).wrapping_mul(element_size),
    );
    return 1 as libc::c_int != 0;
}
