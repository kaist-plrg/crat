use ::libc;
extern "C" {
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
pub unsafe extern "C" fn rpl_ioctl(
    mut fd: libc::c_int,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    buf = args_0.arg::<*mut libc::c_void>();
    return ioctl(fd, request as libc::c_uint as libc::c_ulong, buf);
}
