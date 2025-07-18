use ::libc;
extern "C" {
    fn printSuccess(message: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn snoopy_action_log_syscall_exec();
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_log_syscall_exec_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `action` :: Subunit 'log-syscall-exec'\n\nDescription:\n    Mocks src/action/log-syscall-exec.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit action log-syscall-exec\n    snoopy-test unit action log-syscall-exec --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_log_syscall_exec(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize);
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_unit_action_log_syscall_exec_showHelp();
        return 0 as libc::c_int;
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    CFG = snoopy_configuration_get();
    (*CFG)
        .message_format = b"Test error message, sent out via stdout output.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*CFG).filter_chain = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*CFG).output = b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    snoopy_action_log_syscall_exec();
    snoopy_entrypoint_test_cli_exit();
    printSuccess(
        b"Mocking src/action/log-syscall-exec.c complete.\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
