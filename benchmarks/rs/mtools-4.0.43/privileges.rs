use ::libc;
extern "C" {
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub static mut noPrivileges: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn print_privs(mut message: *const libc::c_char) {}
static mut rgid: gid_t = 0;
static mut egid: gid_t = 0;
static mut ruid: uid_t = 0;
static mut euid: uid_t = 0;
#[inline]
unsafe extern "C" fn Setuid(mut uid: uid_t) {
    if euid == 0 as libc::c_int as libc::c_uint {
        seteuid(uid);
    } else {
        setuid(uid);
    };
}
pub unsafe extern "C" fn reclaim_privs() {
    if noPrivileges != 0 {
        return;
    }
    setgid(egid);
    Setuid(euid);
    print_privs(
        b"after reclaim privs, both uids should be 0 \0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn drop_privs() {
    Setuid(ruid);
    setgid(rgid);
    print_privs(
        b"after drop_privs, real should be 0, effective should not \0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn destroy_privs() {
    if euid == 0 as libc::c_int as libc::c_uint {
        setuid(0 as libc::c_int as __uid_t);
        setuid(ruid);
        seteuid(ruid);
    }
    drop_privs();
    print_privs(
        b"destroy_privs, no uid should be zero  \0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn get_real_uid() -> uid_t {
    return ruid;
}
pub unsafe extern "C" fn init_privs() {
    euid = geteuid();
    ruid = getuid();
    egid = getegid();
    rgid = getgid();
    if euid != ruid {
        unsetenv(b"SOURCE_DATE_EPOCH\0" as *const u8 as *const libc::c_char);
    }
    if euid == 0 as libc::c_int as libc::c_uint
        && ruid != 0 as libc::c_int as libc::c_uint
    {
        setuid(0 as libc::c_int as __uid_t);
    }
    drop_privs();
    print_privs(
        b"after init, real should be 0, effective should not \0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn closeExec(mut fd: libc::c_int) {
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
}
