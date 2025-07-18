use ::libc;
extern "C" {
    fn freecon(con: *mut libc::c_char);
    fn __errno_location() -> *mut libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn getfilecon(file: *const libc::c_char, con: *mut *mut libc::c_char) -> libc::c_int;
    fn lgetfilecon(
        file: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn fgetfilecon(fd: libc::c_int, con: *mut *mut libc::c_char) -> libc::c_int;
}
unsafe extern "C" fn map_to_failure(
    mut ret: libc::c_int,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    if ret == 0 as libc::c_int {
        *__errno_location() = 95 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if ret == 10 as libc::c_int
        && strcmp(*con, b"unlabeled\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        freecon(*con);
        *con = 0 as *mut libc::c_char;
        *__errno_location() = 61 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return ret;
}
pub unsafe extern "C" fn rpl_getfilecon(
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = getfilecon(file, con);
    return map_to_failure(ret, con);
}
pub unsafe extern "C" fn rpl_lgetfilecon(
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = lgetfilecon(file, con);
    return map_to_failure(ret, con);
}
pub unsafe extern "C" fn rpl_fgetfilecon(
    mut fd: libc::c_int,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = fgetfilecon(fd, con);
    return map_to_failure(ret, con);
}
