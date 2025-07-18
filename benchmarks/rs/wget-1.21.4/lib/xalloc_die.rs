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
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub unsafe extern "C" fn xalloc_die() {
    error(
        exit_failure,
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            b"wget-gnulib\0" as *const u8 as *const libc::c_char,
            b"memory exhausted\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    abort();
}
