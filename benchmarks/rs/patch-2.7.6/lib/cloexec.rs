use ::libc;
extern "C" {
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
pub unsafe extern "C" fn set_cloexec_flag(
    mut desc: libc::c_int,
    mut value: bool,
) -> libc::c_int {
    let mut flags: libc::c_int = rpl_fcntl(desc, 1 as libc::c_int, 0 as libc::c_int);
    if 0 as libc::c_int <= flags {
        let mut newflags: libc::c_int = if value as libc::c_int != 0 {
            flags | 1 as libc::c_int
        } else {
            flags & !(1 as libc::c_int)
        };
        if flags == newflags
            || rpl_fcntl(desc, 2 as libc::c_int, newflags) != -(1 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn dup_cloexec(mut fd: libc::c_int) -> libc::c_int {
    return rpl_fcntl(fd, 1030 as libc::c_int, 0 as libc::c_int);
}
