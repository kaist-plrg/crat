use ::libc;
pub unsafe extern "C" fn snoopy_filter_noop(arg: *const libc::c_char) -> libc::c_int {
    return 1 as libc::c_int;
}
