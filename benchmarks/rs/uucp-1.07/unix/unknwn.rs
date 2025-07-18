use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn ixsspawn(
        pazargs: *mut *const libc::c_char,
        aidescs: *mut libc::c_int,
        fkeepuid: boolean,
        fkeepenv: boolean,
        zchdir: *const libc::c_char,
        fnosigs: boolean,
        fshell: boolean,
        zpath: *const libc::c_char,
        zuu_machine: *const libc::c_char,
        zuu_user: *const libc::c_char,
    ) -> pid_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn fsysdep_unknown_caller(
    mut zscript: *const libc::c_char,
    mut zsystem: *const libc::c_char,
) -> boolean {
    let mut azargs: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut ipid: pid_t = 0;
    azargs[0 as libc::c_int as usize] = zscript;
    azargs[1 as libc::c_int as usize] = zsystem;
    azargs[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    aidescs[0 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[1 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[2 as libc::c_int as usize] = -(1 as libc::c_int);
    ipid = ixsspawn(
        azargs.as_mut_ptr(),
        aidescs.as_mut_ptr(),
        1 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if ipid < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ixsspawn: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return (ixswait(ipid as libc::c_ulong, 0 as *mut libc::c_void as *const libc::c_char)
        != 0 as libc::c_int) as libc::c_int;
}
