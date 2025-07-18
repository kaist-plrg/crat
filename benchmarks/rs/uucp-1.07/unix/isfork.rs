use ::libc;
extern "C" {
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub unsafe extern "C" fn ixsfork() -> pid_t {
    let mut i: libc::c_int = 0;
    let mut iret: pid_t = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        iret = fork();
        if iret >= 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
            return iret;
        }
        sleep(5 as libc::c_int as libc::c_uint);
        i += 1;
        i;
    }
    return iret;
}
