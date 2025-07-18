use ::libc;
pub unsafe extern "C" fn snoopy_datasource_noop(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
