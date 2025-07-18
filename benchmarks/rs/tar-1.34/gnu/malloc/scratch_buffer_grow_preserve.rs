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
unsafe extern "C" fn scratch_buffer_init(mut buffer: *mut scratch_buffer) {
    (*buffer).data = ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void;
    (*buffer).length = ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong;
}
pub unsafe extern "C" fn gl_scratch_buffer_grow_preserve(
    mut buffer: *mut scratch_buffer,
) -> bool {
    let mut new_length: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul((*buffer).length);
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*buffer).data == ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        new_ptr = malloc(new_length);
        if new_ptr.is_null() {
            return 0 as libc::c_int != 0;
        }
        memcpy(
            new_ptr,
            ((*buffer).__space.__c).as_mut_ptr() as *const libc::c_void,
            (*buffer).length,
        );
    } else {
        if (new_length >= (*buffer).length) as libc::c_int as libc::c_long != 0 {
            new_ptr = realloc((*buffer).data, new_length);
        } else {
            *__errno_location() = 12 as libc::c_int;
            new_ptr = 0 as *mut libc::c_void;
        }
        if (new_ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
            rpl_free((*buffer).data);
            scratch_buffer_init(buffer);
            return 0 as libc::c_int != 0;
        }
    }
    (*buffer).data = new_ptr;
    (*buffer).length = new_length;
    return 1 as libc::c_int != 0;
}
