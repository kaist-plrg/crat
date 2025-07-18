use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub type va_list = __builtin_va_list;
pub unsafe extern "C" fn xstrdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        NOTIFY(ERROR, b"string has no value!\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    ret = strdup(str);
    if ret.is_null() {
        NOTIFY(
            FATAL,
            b"xstrdup: unable to allocate additional memory\0" as *const u8
                as *const libc::c_char,
        );
    }
    return ret;
}
pub unsafe extern "C" fn xstrcat(
    mut arg1: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut argptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut resptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut valist: ::std::ffi::VaListImpl;
    valist = args.clone();
    argptr = arg1;
    while !argptr.is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen(argptr)) as size_t as size_t;
        argptr = valist.arg::<*mut libc::c_char>();
    }
    result = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    resptr = result;
    valist = args.clone();
    argptr = arg1;
    while !argptr.is_null() {
        len = strlen(argptr);
        memcpy(resptr as *mut libc::c_void, argptr as *const libc::c_void, len);
        resptr = resptr.offset(len as isize);
        argptr = valist.arg::<*mut libc::c_char>();
    }
    *resptr = '\0' as i32 as libc::c_char;
    return result;
}
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if !ptr.is_null() {
        tmp = realloc(ptr, size);
    } else {
        tmp = malloc(size);
    }
    if tmp.is_null() {
        NOTIFY(
            FATAL,
            b"Memory exhausted; unable to continue.\0" as *const u8
                as *const libc::c_char,
        );
    }
    return tmp;
}
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = malloc(size);
    if tmp.is_null() {
        NOTIFY(
            FATAL,
            b"Unable to allocate additional memory.\0" as *const u8
                as *const libc::c_char,
        );
    }
    return tmp;
}
pub unsafe extern "C" fn xcalloc(
    mut num: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = xmalloc(num.wrapping_mul(size));
    memset(tmp, 0 as libc::c_int, num.wrapping_mul(size));
    return tmp;
}
pub unsafe extern "C" fn xfree(mut ptr: *mut libc::c_void) {
    if ptr != 0 as *mut libc::c_void {
        free(ptr);
        ptr = 0 as *mut libc::c_void;
    }
}
