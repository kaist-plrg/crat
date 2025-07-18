use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> libc::c_int;
    fn setregid(__rgid: __gid_t, __egid: __gid_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type uid_t = __uid_t;
pub type gid_t = __gid_t;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn fsuser_perms(
    mut pieuid: *mut uid_t,
    mut piegid: *mut gid_t,
) -> boolean {
    let mut ieuid: uid_t = 0;
    let mut iuid: uid_t = 0;
    let mut iegid: gid_t = 0;
    let mut igid: gid_t = 0;
    ieuid = geteuid();
    iuid = getuid();
    if !pieuid.is_null() {
        *pieuid = ieuid;
    }
    iegid = getegid();
    igid = getgid();
    if !piegid.is_null() {
        *piegid = iegid;
    }
    if setregid(iegid, igid) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"setregid (%ld, %ld): %s\0" as *const u8 as *const libc::c_char,
            iegid as libc::c_long,
            igid as libc::c_long,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    if setreuid(ieuid, iuid) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"setreuid (%ld, %ld): %s\0" as *const u8 as *const libc::c_char,
            ieuid as libc::c_long,
            iuid as libc::c_long,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fsuucp_perms(
    mut ieuid: libc::c_long,
    mut iegid: libc::c_long,
) -> boolean {
    if fsuser_perms(
        0 as *mut libc::c_void as *mut uid_t,
        0 as *mut libc::c_void as *mut gid_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
