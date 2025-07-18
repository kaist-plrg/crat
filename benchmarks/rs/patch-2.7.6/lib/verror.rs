use ::libc;
extern "C" {
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn error_at_line(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __fname: *const libc::c_char,
        __lineno: libc::c_uint,
        __format: *const libc::c_char,
        _: ...
    );
    fn xvasprintf(
        format: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub unsafe extern "C" fn verror(
    mut status: libc::c_int,
    mut errnum: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    verror_at_line(
        status,
        errnum,
        0 as *const libc::c_char,
        0 as libc::c_int as libc::c_uint,
        format,
        args.as_va_list(),
    );
}
pub unsafe extern "C" fn verror_at_line(
    mut status: libc::c_int,
    mut errnum: libc::c_int,
    mut file: *const libc::c_char,
    mut line_number: libc::c_uint,
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut message: *mut libc::c_char = xvasprintf(format, args.as_va_list());
    if !message.is_null() {
        if !file.is_null() {
            error_at_line(
                status,
                errnum,
                file,
                line_number,
                b"%s\0" as *const u8 as *const libc::c_char,
                message,
            );
        } else {
            error(status, errnum, b"%s\0" as *const u8 as *const libc::c_char, message);
        }
    } else {
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"unable to display error message\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    free(message as *mut libc::c_void);
}
