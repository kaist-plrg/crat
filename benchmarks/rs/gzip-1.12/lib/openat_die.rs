use ::libc;
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
}
pub unsafe extern "C" fn openat_save_fail(mut errnum: libc::c_int) {
    error(
        exit_failure,
        errnum,
        b"unable to record current working directory\0" as *const u8
            as *const libc::c_char,
    );
    abort();
}
pub unsafe extern "C" fn openat_restore_fail(mut errnum: libc::c_int) {
    error(
        exit_failure,
        errnum,
        b"failed to return to initial working directory\0" as *const u8
            as *const libc::c_char,
    );
    abort();
}
