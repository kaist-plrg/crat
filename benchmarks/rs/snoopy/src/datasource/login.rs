use ::libc;
extern "C" {
    fn getlogin_r(__name: *mut libc::c_char, __name_len: size_t) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_datasource_login(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut loginSizeMaxWithoutNull: libc::c_int = 254 as libc::c_int;
    let mut loginSizeMaxWithNull: libc::c_int = 255 as libc::c_int;
    static mut login: [libc::c_char; 255] = [0; 255];
    let mut loginptr: *const libc::c_char = 0 as *const libc::c_char;
    if 0 as libc::c_int != getlogin_r(login.as_mut_ptr(), loginSizeMaxWithNull as size_t)
    {
        loginptr = getenv(b"SUDO_USER\0" as *const u8 as *const libc::c_char);
        if loginptr.is_null() {
            loginptr = getenv(b"LOGNAME\0" as *const u8 as *const libc::c_char);
        }
        if loginptr.is_null() {
            strcpy(
                login.as_mut_ptr(),
                b"(unknown)\0" as *const u8 as *const libc::c_char,
            );
        } else {
            strncpy(
                login.as_mut_ptr(),
                loginptr,
                loginSizeMaxWithoutNull as libc::c_ulong,
            );
            if strlen(loginptr) as libc::c_int > loginSizeMaxWithoutNull {
                login[loginSizeMaxWithoutNull as usize] = '\0' as i32 as libc::c_char;
            }
        }
    }
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        login.as_mut_ptr(),
    );
}
