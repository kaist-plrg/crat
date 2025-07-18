use ::libc;
extern "C" {
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub unsafe extern "C" fn stdopen() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = 0 as libc::c_int;
    while fd <= 2 as libc::c_int {
        if rpl_fcntl(fd, 1 as libc::c_int) < 0 as libc::c_int {
            let mut mode: libc::c_int = if fd == 0 as libc::c_int {
                0o1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut full_fd: libc::c_int = if fd == 0 as libc::c_int {
                open(b"/dev/full\0" as *const u8 as *const libc::c_char, mode)
            } else {
                -(1 as libc::c_int)
            };
            let mut new_fd: libc::c_int = if full_fd < 0 as libc::c_int {
                open(b"/dev/null\0" as *const u8 as *const libc::c_char, mode)
            } else {
                full_fd
            };
            if new_fd < 0 as libc::c_int {
                return *__errno_location();
            }
            if (2 as libc::c_int) < new_fd {
                close(new_fd);
                return 0 as libc::c_int;
            }
        }
        fd += 1;
        fd;
    }
    return 0 as libc::c_int;
}
