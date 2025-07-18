use ::libc;
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
pub unsafe extern "C" fn free_permission_context(mut ctx: *mut permission_context) {}
