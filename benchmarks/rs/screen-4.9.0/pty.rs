use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn grantpt(__fd: libc::c_int) -> libc::c_int;
    fn unlockpt(__fd: libc::c_int) -> libc::c_int;
    fn ptsname(__fd: libc::c_int) -> *mut libc::c_char;
    fn getpt() -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
static mut PtyName: [libc::c_char; 32] = [0; 32];
static mut TtyName: [libc::c_char; 32] = [0; 32];
pub static mut pty_preopen: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn initmaster(mut f: libc::c_int) {
    tcflush(f, 2 as libc::c_int);
}
pub unsafe extern "C" fn InitPTY(mut f: libc::c_int) {
    if f < 0 as libc::c_int {
        return;
    }
}
pub unsafe extern "C" fn OpenPTY(mut ttyn: *mut *mut libc::c_char) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sigcld: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
    strcpy(PtyName.as_mut_ptr(), b"/dev/ptmx\0" as *const u8 as *const libc::c_char);
    f = getpt();
    if f == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    sigcld = xsignal(17 as libc::c_int, None);
    m = ptsname(f);
    if m.is_null() || grantpt(f) != 0 || unlockpt(f) != 0 {
        xsignal(17 as libc::c_int, sigcld);
        close(f);
        return -(1 as libc::c_int);
    }
    xsignal(17 as libc::c_int, sigcld);
    if strlen(m) < ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong {
        strcpy(TtyName.as_mut_ptr(), m);
    } else {
        close(f);
        return -(1 as libc::c_int);
    }
    initmaster(f);
    *ttyn = TtyName.as_mut_ptr();
    return f;
}
pub unsafe extern "C" fn GetPtsPathOrSymlink(mut fd: libc::c_int) -> *mut libc::c_char {
    let mut ret: libc::c_int = 0;
    let mut tty_name: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut tty_symlink: [libc::c_char; 35] = [0; 35];
    *__errno_location() = 0 as libc::c_int;
    tty_name = ttyname(fd);
    if tty_name.is_null() && *__errno_location() == 19 as libc::c_int {
        ret = snprintf(
            tty_symlink.as_mut_ptr(),
            (14 as libc::c_int + 21 as libc::c_int) as libc::c_ulong,
            b"/proc/self/fd/%d\0" as *const u8 as *const libc::c_char,
            fd,
        );
        if ret < 0 as libc::c_int || ret >= 14 as libc::c_int + 21 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        *__errno_location() = 19 as libc::c_int;
        return tty_symlink.as_mut_ptr();
    }
    return tty_name;
}
