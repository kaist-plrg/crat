use ::libc;
extern "C" {
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub unsafe extern "C" fn pipe_safer(mut fd: *mut libc::c_int) -> libc::c_int {
    if pipe(fd) == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            *fd.offset(i as isize) = fd_safer(*fd.offset(i as isize));
            if *fd.offset(i as isize) < 0 as libc::c_int {
                let mut saved_errno: libc::c_int = *__errno_location();
                close(*fd.offset((1 as libc::c_int - i) as isize));
                *__errno_location() = saved_errno;
                return -(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
