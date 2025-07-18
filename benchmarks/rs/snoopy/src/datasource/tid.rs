use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
}
pub type pthread_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_datasource_tid(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut tid: libc::c_ulong = pthread_self();
    if 0 as libc::c_int as libc::c_ulong == tid {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ pthread_self())\0" as *const u8 as *const libc::c_char,
        );
    }
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        tid,
    );
}
