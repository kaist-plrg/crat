use ::libc;
extern "C" {
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const BILLION: C2RustUnnamed = 1000000000;
pub type C2RustUnnamed = libc::c_uint;
pub unsafe extern "C" fn rpl_nanosleep(
    mut requested_delay: *const timespec,
    mut remaining_delay: *mut timespec,
) -> libc::c_int {
    if (*requested_delay).tv_nsec < 0 as libc::c_int as libc::c_long
        || BILLION as libc::c_int as libc::c_long <= (*requested_delay).tv_nsec
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let limit: time_t = (24 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
        * 60 as libc::c_int) as time_t;
    let mut seconds: time_t = (*requested_delay).tv_sec;
    let mut intermediate: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    intermediate.tv_nsec = (*requested_delay).tv_nsec;
    while limit < seconds {
        let mut result: libc::c_int = 0;
        intermediate.tv_sec = limit;
        result = nanosleep(&mut intermediate, remaining_delay);
        seconds -= limit;
        if result != 0 {
            if !remaining_delay.is_null() {
                (*remaining_delay).tv_sec += seconds;
            }
            return result;
        }
        intermediate.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    }
    intermediate.tv_sec = seconds;
    return nanosleep(&mut intermediate, remaining_delay);
}
