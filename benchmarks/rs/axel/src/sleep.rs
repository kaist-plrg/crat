use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub unsafe extern "C" fn axel_sleep(mut delay: timespec) -> libc::c_int {
    let mut res: libc::c_int = 0;
    loop {
        res = nanosleep(&mut delay, &mut delay);
        if !(res != 0 && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return res;
}
