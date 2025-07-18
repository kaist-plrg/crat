use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
}
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub unsafe extern "C" fn stralloc(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    retval = calloc(
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    if retval.is_null() {
        NOTIFY(
            FATAL,
            b"Fatal memory allocation error\0" as *const u8 as *const libc::c_char,
        );
    }
    strcpy(retval, str);
    return retval;
}
