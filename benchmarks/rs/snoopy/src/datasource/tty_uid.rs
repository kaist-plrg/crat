use ::libc;
extern "C" {
    fn snoopy_datasource_tty__get_tty_uid(
        ttyUid: *mut uid_t,
        result: *mut libc::c_char,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type uid_t = __uid_t;
pub type __uid_t = libc::c_uint;
pub unsafe extern "C" fn snoopy_datasource_tty_uid(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut retVal: libc::c_int = 0;
    let mut ttyUid: uid_t = 0;
    retVal = snoopy_datasource_tty__get_tty_uid(&mut ttyUid, result);
    if retVal > 0 as libc::c_int {
        return retVal;
    }
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        ttyUid,
    );
}
