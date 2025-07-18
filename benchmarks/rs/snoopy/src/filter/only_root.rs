use ::libc;
extern "C" {
    fn getuid() -> __uid_t;
}
pub type __uid_t = libc::c_uint;
pub unsafe extern "C" fn snoopy_filter_only_root(
    arg: *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_uint == getuid() {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
