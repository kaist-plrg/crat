use ::libc;
extern "C" {
    fn nettle_sexp_vformat(
        buffer: *mut nettle_buffer,
        format: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> size_t;
    fn nettle_base64_encode_raw(
        dst: *mut libc::c_char,
        length: size_t,
        src: *const uint8_t,
    );
    fn nettle_buffer_grow(buffer: *mut nettle_buffer, length: size_t) -> libc::c_int;
    fn nettle_buffer_space(buffer: *mut nettle_buffer, length: size_t) -> *mut uint8_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[inline]
unsafe extern "C" fn base64_encode_in_place(mut length: size_t, mut data: *mut uint8_t) {
    nettle_base64_encode_raw(data as *mut libc::c_char, length, data);
}
pub unsafe extern "C" fn nettle_sexp_transport_vformat(
    mut buffer: *mut nettle_buffer,
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> size_t {
    let mut start: size_t = 0 as libc::c_int as size_t;
    let mut length: size_t = 0;
    let mut base64_length: size_t = 0;
    if !buffer.is_null() {
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh0 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh0 as isize) = '{' as i32 as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int as size_t;
        }
        start = (*buffer).size;
    }
    length = nettle_sexp_vformat(buffer, format, args.as_va_list());
    if length == 0 {
        return 0 as libc::c_int as size_t;
    }
    base64_length = length
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        .wrapping_div(3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong);
    if !buffer.is_null() {
        if (nettle_buffer_space(buffer, base64_length.wrapping_sub(length))).is_null() {
            return 0 as libc::c_int as size_t;
        }
        base64_encode_in_place(length, ((*buffer).contents).offset(start as isize));
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh1 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh1 as isize) = '}' as i32 as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int as size_t;
        }
    }
    return base64_length.wrapping_add(2 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn nettle_sexp_transport_format(
    mut buffer: *mut nettle_buffer,
    mut format: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut done: size_t = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    done = nettle_sexp_transport_vformat(buffer, format, args_0.as_va_list());
    return done;
}
