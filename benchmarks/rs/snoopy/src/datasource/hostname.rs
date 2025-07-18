use ::libc;
extern "C" {
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_datasource_hostname(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut charCount: libc::c_int = 0;
    let mut retVal: libc::c_int = 0;
    retVal = gethostname(result, 2048 as libc::c_int as size_t);
    if 0 as libc::c_int != retVal {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ gethostname(): %d)\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
    }
    *result
        .offset(
            (2048 as libc::c_int - 1 as libc::c_int) as isize,
        ) = '\0' as i32 as libc::c_char;
    charCount = strlen(result) as libc::c_int;
    return charCount;
}
