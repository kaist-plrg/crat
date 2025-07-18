use ::libc;
extern "C" {
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
pub unsafe extern "C" fn dup_safer_flag(
    mut fd: libc::c_int,
    mut flag: libc::c_int,
) -> libc::c_int {
    return rpl_fcntl(
        fd,
        if flag & 0o2000000 as libc::c_int != 0 {
            1030 as libc::c_int
        } else {
            0 as libc::c_int
        },
        2 as libc::c_int + 1 as libc::c_int,
    );
}
