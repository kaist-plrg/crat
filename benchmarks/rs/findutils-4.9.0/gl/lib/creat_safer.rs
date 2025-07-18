use ::libc;
extern "C" {
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
pub unsafe extern "C" fn creat_safer(
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return fd_safer(creat(file, mode));
}
