use ::libc;
extern "C" {
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub unsafe extern "C" fn snoopy_datasource_tid_kernel(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut tid: libc::c_ulong = syscall(186 as libc::c_int as libc::c_long)
        as libc::c_ulong;
    if 0 as libc::c_int as libc::c_ulong == tid {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ syscall(SYS_gettid))\0" as *const u8 as *const libc::c_char,
        );
    }
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        tid,
    );
}
