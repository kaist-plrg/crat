use ::libc;
extern "C" {
    fn snoopy_outputregistry_dispatch(logMessage: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub unsafe extern "C" fn snoopy_action_log_message_dispatch(
    mut logMessage: *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int as libc::c_ulong == strlen(logMessage) {
        return 0 as libc::c_int;
    }
    return snoopy_outputregistry_dispatch(logMessage);
}
