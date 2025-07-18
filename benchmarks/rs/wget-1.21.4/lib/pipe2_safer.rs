use ::libc;
extern "C" {
    fn fd_safer_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn rpl_pipe2(fd: *mut libc::c_int, flags: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub unsafe extern "C" fn pipe2_safer(
    mut fd: *mut libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    if rpl_pipe2(fd, flags) == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            *fd.offset(i as isize) = fd_safer_flag(*fd.offset(i as isize), flags);
            if *fd.offset(i as isize) < 0 as libc::c_int {
                let mut e: libc::c_int = *__errno_location();
                close(*fd.offset((1 as libc::c_int - i) as isize));
                *__errno_location() = e;
                return -(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
