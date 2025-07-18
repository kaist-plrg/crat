use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scratch_buffer {
    pub data: *mut libc::c_void,
    pub length: size_t,
    pub __space: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __align: max_align_t,
    pub __c: [libc::c_char; 1024],
}
#[inline]
unsafe extern "C" fn scratch_buffer_free(mut buffer: *mut scratch_buffer) {
    if (*buffer).data != ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        rpl_free((*buffer).data);
    }
}
#[inline]
unsafe extern "C" fn scratch_buffer_init(mut buffer: *mut scratch_buffer) {
    (*buffer).data = ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void;
    (*buffer).length = ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong;
}
pub unsafe extern "C" fn gl_scratch_buffer_set_array_size(
    mut buffer: *mut scratch_buffer,
    mut nelem: size_t,
    mut size: size_t,
) -> bool {
    let mut new_length: size_t = nelem.wrapping_mul(size);
    if (nelem | size)
        >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
        && nelem != 0 as libc::c_int as libc::c_ulong
        && size != new_length.wrapping_div(nelem)
    {
        scratch_buffer_free(buffer);
        scratch_buffer_init(buffer);
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    if new_length <= (*buffer).length {
        return 1 as libc::c_int != 0;
    }
    scratch_buffer_free(buffer);
    let mut new_ptr: *mut libc::c_char = malloc(new_length) as *mut libc::c_char;
    if new_ptr.is_null() {
        scratch_buffer_init(buffer);
        return 0 as libc::c_int != 0;
    }
    (*buffer).data = new_ptr as *mut libc::c_void;
    (*buffer).length = new_length;
    return 1 as libc::c_int != 0;
}
