use ::libc;
extern "C" {
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub text: [libc::c_char; 160],
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub source: [libc::c_char; 80],
}
pub unsafe extern "C" fn jsonp_error_init(
    mut error: *mut json_error_t,
    mut source: *const libc::c_char,
) {
    if !error.is_null() {
        (*error).text[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*error).line = -(1 as libc::c_int);
        (*error).column = -(1 as libc::c_int);
        strncpy(
            ((*error).source).as_mut_ptr(),
            source,
            80 as libc::c_int as libc::c_ulong,
        );
        (*error)
            .source[(80 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn jsonp_error_set(
    mut error: *mut json_error_t,
    mut line: libc::c_int,
    mut column: libc::c_int,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    if error.is_null() {
        return;
    }
    if (*error).text[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        return;
    }
    (*error).line = line;
    (*error).column = column;
    ap = args.clone();
    vsnprintf(
        ((*error).text).as_mut_ptr(),
        160 as libc::c_int as libc::c_ulong,
        msg,
        ap.as_va_list(),
    );
}
