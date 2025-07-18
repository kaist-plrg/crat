use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_message_generateFromFormat(
        logMessage: *mut libc::c_char,
        logMessageBufSize: size_t,
        logMessageFormat: *mut libc::c_char,
    );
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopyTestCli_action_run_messageformat_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `message formatter`\n\nUsage:\n    snoopy-test run messageformat \"FORMAT SPECIFICATION\"\n\nResult:\n    Prints a log message formatted according to the given format specification.\n    Process data is taken from self.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_messageformat(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut messageFormat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_messageformat_showHelp();
        fatalError(
            b"Missing argument: message format\0" as *const u8 as *const libc::c_char,
        );
    }
    messageFormat = *argv.offset(0 as libc::c_int as isize);
    message = malloc(16383 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    *message.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    snoopy_message_generateFromFormat(
        message,
        16383 as libc::c_int as size_t,
        messageFormat,
    );
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, message);
    free(message as *mut libc::c_void);
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
