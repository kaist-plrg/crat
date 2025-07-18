use ::libc;
extern "C" {
    fn printSuccess(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn snoopy_outputregistry_getCount() -> libc::c_int;
    fn snoopy_outputregistry_doesIdExist(outputId: libc::c_int) -> libc::c_int;
    fn snoopy_outputregistry_getIdFromName(
        outputName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_outputregistry_getName(outputId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_outputregistry_callById(
        outputId: libc::c_int,
        logMessage: *const libc::c_char,
        outputArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_outputregistry_callByName(
        outputName: *const libc::c_char,
        logMessage: *const libc::c_char,
        outputArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_outputregistry_dispatch(logMessage: *const libc::c_char) -> libc::c_int;
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
pub unsafe extern "C" fn snoopyTestCli_action_unit_outputregistry_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `outputregistry`\n\nDescription:\n    Mocks outputregistry implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit outputregistry\n    snoopy-test unit outputregistry --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_outputregistry(
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
        snoopyTestCli_action_unit_outputregistry_showHelp();
        return 0 as libc::c_int;
    }
    let mut outputName: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputCount: libc::c_int = 0 as libc::c_int;
    let mut outputIdPreset: libc::c_int = 0 as libc::c_int;
    let mut outputIdReceived: libc::c_int = 0 as libc::c_int;
    outputCount = snoopy_outputregistry_getCount();
    if outputCount < 1 as libc::c_int {
        fatalError(
            b"No outputs available, output count is 0\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(
        b"[DEBUG] Number of outputs available: %d\n\0" as *const u8
            as *const libc::c_char,
        outputCount,
    );
    if snoopy_outputregistry_doesIdExist(outputIdPreset) != 1 as libc::c_int {
        fatalError(
            b"Output with a preset ID (0) does not exist\0" as *const u8
                as *const libc::c_char,
        );
    }
    outputName = snoopy_outputregistry_getName(outputIdPreset);
    outputIdReceived = snoopy_outputregistry_getIdFromName(outputName);
    if outputIdPreset != outputIdReceived {
        fatalError(
            b"Output ID returned when searching by name differs from the initially used ID to find that same output\0"
                as *const u8 as *const libc::c_char,
        );
    }
    outputName = b"noop\0" as *const u8 as *const libc::c_char;
    outputIdReceived = snoopy_outputregistry_getIdFromName(outputName);
    snoopy_outputregistry_callById(
        outputIdReceived,
        0 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if -(1 as libc::c_int)
        != snoopy_outputregistry_callById(
            -(1 as libc::c_int),
            0 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalError(
            b"Output ID -1 unexpectedly exists\0" as *const u8 as *const libc::c_char,
        );
    }
    if -(1 as libc::c_int)
        != snoopy_outputregistry_callByName(
            b"fakeOutputNameThatShouldNeverExist\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalError(
            b"Output with an unexpected name actually exists\0" as *const u8
                as *const libc::c_char,
        );
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    CFG = snoopy_configuration_get();
    (*CFG).output = b"noop\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    snoopy_outputregistry_dispatch(0 as *const libc::c_char);
    snoopy_entrypoint_test_cli_exit();
    printSuccess(
        b"Mocking src/outputregistry.c complete.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
