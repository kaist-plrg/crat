use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type Sfdisc_t = *mut libc::c_void;
pub unsafe extern "C" fn pth_sfiodisc() -> *mut Sfdisc_t {
    *__errno_location() = 38 as libc::c_int;
    return 0 as *mut libc::c_void as *mut Sfdisc_t;
}
