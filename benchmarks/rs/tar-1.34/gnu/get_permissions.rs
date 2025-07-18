use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
pub unsafe extern "C" fn get_permissions(
    mut name: *const libc::c_char,
    mut desc: libc::c_int,
    mut mode: mode_t,
    mut ctx: *mut permission_context,
) -> libc::c_int {
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<permission_context>() as libc::c_ulong,
    );
    (*ctx).mode = mode;
    return 0 as libc::c_int;
}
