use ::libc;
extern "C" {
    fn getegid() -> __gid_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __gid_t = libc::c_uint;
pub unsafe extern "C" fn snoopy_datasource_egid(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        getegid(),
    );
}
