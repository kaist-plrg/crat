use ::libc;
extern "C" {
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type pollfd_struct = pollfd;
pub unsafe extern "C" fn mypoll(
    mut fds: *mut pollfd_struct,
    mut nfds: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nfds {
        if (*fds.offset(i as isize)).events == 0 {
            (*fds.offset(i as isize)).fd = !(*fds.offset(i as isize)).fd;
        }
        i += 1;
        i;
    }
    ret = poll(
        fds,
        nfds as nfds_t,
        if timeout <= 0 as libc::c_int { timeout } else { timeout * 1000 as libc::c_int },
    );
    i = 0 as libc::c_int;
    while i < nfds {
        if (*fds.offset(i as isize)).events == 0 {
            (*fds.offset(i as isize)).fd = !(*fds.offset(i as isize)).fd;
        }
        i += 1;
        i;
    }
    return ret;
}
