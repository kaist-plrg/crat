use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn vasnprintf(
        resultbuf: *mut libc::c_char,
        lengthp: *mut size_t,
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type uintptr_t = libc::c_ulong;
pub unsafe extern "C" fn rpl_sprintf(
    mut str: *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut lenbuf: size_t = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    lenbuf = if (18446744073709551615 as libc::c_ulong)
        < 2147483647 as libc::c_int as libc::c_ulong
    {
        18446744073709551615 as libc::c_ulong
    } else {
        2147483647 as libc::c_int as libc::c_ulong
    };
    if lenbuf > !(str as uintptr_t) {
        lenbuf = !(str as uintptr_t);
    }
    args_0 = args.clone();
    output = vasnprintf(str, &mut lenbuf, format, args_0.as_va_list());
    len = lenbuf;
    if output.is_null() {
        return -(1 as libc::c_int);
    }
    if output != str {
        rpl_free(output as *mut libc::c_void);
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if len > 2147483647 as libc::c_int as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return len as libc::c_int;
}
