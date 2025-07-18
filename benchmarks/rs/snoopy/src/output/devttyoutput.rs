use ::libc;
extern "C" {
    fn snoopy_output_fileoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
}
pub unsafe extern "C" fn snoopy_output_devttyoutput(
    logMessage: *const libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    return snoopy_output_fileoutput(
        logMessage,
        b"/dev/tty\0" as *const u8 as *const libc::c_char,
    );
}
