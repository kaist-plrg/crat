use ::libc;
extern "C" {
    fn stdopen() -> libc::c_int;
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
pub unsafe extern "C" fn xstdopen() {
    let mut stdopen_errno: libc::c_int = stdopen();
    if stdopen_errno != 0 as libc::c_int {
        error(
            exit_failure,
            stdopen_errno,
            dcgettext(
                0 as *const libc::c_char,
                b"standard file descriptors\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
