use ::libc;
extern "C" {
    fn set_permissions(
        _: *mut permission_context,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn get_permissions(
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
        _: *mut permission_context,
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
pub unsafe extern "C" fn qcopy_acl(
    mut src_name: *const libc::c_char,
    mut source_desc: libc::c_int,
    mut dst_name: *const libc::c_char,
    mut dest_desc: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ctx: permission_context = permission_context { mode: 0 };
    let mut ret: libc::c_int = 0;
    ret = get_permissions(src_name, source_desc, mode, &mut ctx);
    if ret != 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    ret = set_permissions(&mut ctx, dst_name, dest_desc);
    free_permission_context(&mut ctx);
    return ret;
}
