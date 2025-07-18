use ::libc;
extern "C" {
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn xcalloc(
    mut count: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = calloc(count, size);
    if res.is_null() {
        die();
    }
    return res;
}
pub unsafe extern "C" fn xfree(mut ptr: *mut libc::c_void) {
    free(ptr);
}
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = malloc(size);
    if res.is_null() {
        die();
    }
    memset(res, 0 as libc::c_int, size);
    return res;
}
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = realloc(ptr, size);
    if res.is_null() {
        die();
    }
    return res;
}
unsafe extern "C" fn die() -> ! {
    log_fatal(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Out of memory\0" as *const u8 as *const libc::c_char,
    );
}
