use ::libc;
extern "C" {
    fn snoopy_action_log_message_dispatch(
        logMessage: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_filtering_check_chain(chain: *const libc::c_char) -> libc::c_int;
    fn snoopy_message_generateFromFormat(
        logMessage: *mut libc::c_char,
        logMessageBufSize: size_t,
        logMessageFormat: *mut libc::c_char,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_action_log_syscall_exec() {
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    let mut logMessage: *mut libc::c_char = 0 as *mut libc::c_char;
    CFG = snoopy_configuration_get();
    if 1 as libc::c_int == (*CFG).filtering_enabled
        && 0 as libc::c_int == snoopy_filtering_check_chain((*CFG).filter_chain)
    {
        return;
    }
    logMessage = malloc(16383 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    *logMessage.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    snoopy_message_generateFromFormat(
        logMessage,
        16383 as libc::c_int as size_t,
        (*CFG).message_format,
    );
    snoopy_action_log_message_dispatch(logMessage);
    free(logMessage as *mut libc::c_void);
}
