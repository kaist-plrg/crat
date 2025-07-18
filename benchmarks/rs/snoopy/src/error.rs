use ::libc;
extern "C" {
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_action_log_message_dispatch(
        logMessage: *const libc::c_char,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_configuration_t {
    pub initialized: libc::c_int,
    pub configfile_enabled: libc::c_int,
    pub configfile_path: *mut libc::c_char,
    pub configfile_found: libc::c_int,
    pub configfile_parsed: libc::c_int,
    pub error_logging_enabled: libc::c_int,
    pub message_format: *mut libc::c_char,
    pub message_format_malloced: libc::c_int,
    pub filtering_enabled: libc::c_int,
    pub filter_chain: *mut libc::c_char,
    pub filter_chain_malloced: libc::c_int,
    pub output: *mut libc::c_char,
    pub output_malloced: libc::c_int,
    pub output_arg: *mut libc::c_char,
    pub output_arg_malloced: libc::c_int,
    pub syslog_facility: libc::c_int,
    pub syslog_level: libc::c_int,
    pub syslog_ident_format_malloced: libc::c_int,
    pub syslog_ident_format: *mut libc::c_char,
}
pub unsafe extern "C" fn snoopy_error_handler(errorMsg: *const libc::c_char) {
    let mut CFG: *const snoopy_configuration_t = 0 as *const snoopy_configuration_t;
    let mut errorMsgFormatted: [libc::c_char; 4096] = [0; 4096];
    errorMsgFormatted[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    CFG = snoopy_configuration_get();
    if 1 as libc::c_int != (*CFG).error_logging_enabled {
        return;
    }
    snprintf(
        errorMsgFormatted.as_mut_ptr(),
        4096 as libc::c_int as libc::c_ulong,
        b"SNOOPY ERROR: %s\0" as *const u8 as *const libc::c_char,
        errorMsg,
    );
    errorMsgFormatted[(4096 as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    snoopy_action_log_message_dispatch(errorMsg);
}
