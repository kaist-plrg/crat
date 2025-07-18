use ::libc;
extern "C" {
    fn snoopy_datasource_tty__get_tty_uid(
        ttyUid: *mut uid_t,
        result: *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_pwd_convertUidToUsername(uid: uid_t) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
}
pub type uid_t = __uid_t;
pub type __uid_t = libc::c_uint;
pub unsafe extern "C" fn snoopy_datasource_tty_username(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut retVal: libc::c_int = 0;
    let mut ttyUid: uid_t = 0;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retMsgLen: libc::c_int = 0 as libc::c_int;
    retVal = snoopy_datasource_tty__get_tty_uid(&mut ttyUid, result);
    if retVal > 0 as libc::c_int {
        return retVal;
    }
    username = snoopy_util_pwd_convertUidToUsername(ttyUid);
    if username.is_null() {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"Unable to convert UID to username\0" as *const u8 as *const libc::c_char,
        );
    }
    retMsgLen = snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        username,
    );
    free(username as *mut libc::c_void);
    return retMsgLen;
}
