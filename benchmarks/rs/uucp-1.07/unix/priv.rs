use ::libc;
extern "C" {
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
}
pub type __uid_t = libc::c_uint;
pub type uid_t = __uid_t;
pub type boolean = libc::c_int;
pub unsafe extern "C" fn fsysdep_privileged() -> boolean {
    let mut iuid: uid_t = 0;
    iuid = getuid();
    return (iuid == 0 as libc::c_int as libc::c_uint || iuid == geteuid())
        as libc::c_int;
}
