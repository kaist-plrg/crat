use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gl_scratch_buffer_dupfree(
    mut buffer: *mut scratch_buffer,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = (*buffer).data;
    if data == ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        let mut copy: *mut libc::c_void = malloc(size);
        return if !copy.is_null() {
            memcpy(copy, data, size)
        } else {
            0 as *mut libc::c_void
        };
    } else {
        let mut copy_0: *mut libc::c_void = realloc(data, size);
        return if !copy_0.is_null() { copy_0 } else { data };
    };
}
