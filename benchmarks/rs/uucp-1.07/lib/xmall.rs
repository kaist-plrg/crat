use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn xmalloc(mut c: size_t) -> pointer {
    let mut pret: pointer = 0 as *mut libc::c_void;
    pret = malloc(c);
    if pret.is_null() && c != 0 as libc::c_int as libc::c_ulong {
        ulog(LOG_FATAL, b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    return pret;
}
