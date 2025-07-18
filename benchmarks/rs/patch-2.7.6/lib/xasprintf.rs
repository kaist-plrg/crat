use ::libc;
extern "C" {
    fn xvasprintf(
        format: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> *mut libc::c_char;
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
pub unsafe extern "C" fn xasprintf(
    mut format: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    args_0 = args.clone();
    result = xvasprintf(format, args_0.as_va_list());
    return result;
}
