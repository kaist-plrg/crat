use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_close();
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn _exit(_: libc::c_int) -> !;
    fn ixsfork() -> pid_t;
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
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn usysdep_detach();
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn fsysdep_run(
    mut ffork: boolean,
    mut zprogram: *const libc::c_char,
    mut zarg1: *const libc::c_char,
    mut zarg2: *const libc::c_char,
) -> boolean {
    let mut zlib: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut azargs: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut ipid: pid_t = 0;
    if ffork != 0 {
        ipid = ixsfork();
        if ipid < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"fork: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        if ipid != 0 as libc::c_int {
            ixswait(
                ipid as libc::c_ulong,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            ulog_close();
            return 1 as libc::c_int;
        }
        usysdep_detach();
    }
    zlib = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_add(strlen(zprogram)),
    );
    sprintf(
        zlib,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        b"/usr/sbin\0" as *const u8 as *const libc::c_char,
        zprogram,
    );
    azargs[0 as libc::c_int as usize] = zlib;
    azargs[1 as libc::c_int as usize] = zarg1;
    azargs[2 as libc::c_int as usize] = zarg2;
    azargs[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    aidescs[0 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[1 as libc::c_int as usize] = -(1 as libc::c_int);
    aidescs[2 as libc::c_int as usize] = -(1 as libc::c_int);
    ipid = ixsspawn(
        azargs.as_mut_ptr(),
        aidescs.as_mut_ptr(),
        1 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    ubuffree(zlib);
    if ipid < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ixsspawn: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if ffork != 0 {
            _exit(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if ffork != 0 {
        _exit(0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
