use ::libc;
pub unsafe extern "C" fn snoopy_output_noopoutput(
    logMessage: *const libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
