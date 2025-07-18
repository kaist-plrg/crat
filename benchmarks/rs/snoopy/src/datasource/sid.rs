use ::libc;
extern "C" {
    fn getsid(__pid: __pid_t) -> __pid_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __pid_t = libc::c_int;
pub unsafe extern "C" fn snoopy_datasource_sid(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        getsid(0 as libc::c_int),
    );
}
