use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn set_permissions(
        _: *mut permission_context,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn free_permission_context(_: *mut permission_context);
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
pub unsafe extern "C" fn qset_acl(
    mut name: *const libc::c_char,
    mut desc: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ctx: permission_context = permission_context { mode: 0 };
    let mut ret: libc::c_int = 0;
    memset(
        &mut ctx as *mut permission_context as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<permission_context>() as libc::c_ulong,
    );
    ctx.mode = mode;
    ret = set_permissions(&mut ctx, name, desc);
    free_permission_context(&mut ctx);
    return ret;
}
