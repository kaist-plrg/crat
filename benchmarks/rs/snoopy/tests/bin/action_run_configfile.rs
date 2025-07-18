use ::libc;
extern "C" {
    fn printErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_util_syslog_convertFacilityToStr(
        facilityInt: libc::c_int,
    ) -> *const libc::c_char;
    fn snoopy_util_syslog_convertLevelToStr(
        levelInt: libc::c_int,
    ) -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
pub unsafe extern "C" fn displayHelp() {
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Usage: \n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    snoopy-test-configfile PATH-TO-INI CONFIG-VARIABLE-TO-DISPLAY\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Available configfile variables:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    (check etc/snoopy.ini for list of supported configuration variables)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_configfile_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `configfile`\n\nUsage:\n    snoopy-test run configfile INI_FILE KEY\n\nResult:\n    Prints value of the requested configuration KEY from the given INI_FILE.\n\nSupported configuration keys (check etc/snoopy.ini for more information):\n    message_format\n    filter_chain\n    output\n    syslog_facility\n    syslog_ident\n    syslog_level\nNOTICE: These keys MUST be placed in a section named [snoopy].\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_configfile(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut iniFilePath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut showConfigVar: *const libc::c_char = 0 as *const libc::c_char;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_configfile_showHelp();
        fatalError(
            b"Missing argument: path to INI config file\0" as *const u8
                as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        )
    {
        snoopyTestCli_action_run_configfile_showHelp();
        return 0 as libc::c_int;
    }
    iniFilePath = *argv.offset(0 as libc::c_int as isize);
    if argc < 2 as libc::c_int {
        snoopyTestCli_action_run_configfile_showHelp();
        fatalError(
            b"Missing argument: configuration variable to display\0" as *const u8
                as *const libc::c_char,
        );
    }
    showConfigVar = *argv.offset(1 as libc::c_int as isize);
    if -(1 as libc::c_int) == access(iniFilePath, 4 as libc::c_int) {
        snoopyTestCli_action_run_configfile_showHelp();
        printErrorValue(
            b"INI file path\0" as *const u8 as *const libc::c_char,
            iniFilePath,
        );
        fatalErrorValue(
            b"Unable to open/read given INI file\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        iniFilePath,
    );
    CFG = snoopy_configuration_get();
    if 0 as libc::c_int
        == strcmp(showConfigVar, b"message_format\0" as *const u8 as *const libc::c_char)
    {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*CFG).message_format);
    } else if 0 as libc::c_int
        == strcmp(showConfigVar, b"filter_chain\0" as *const u8 as *const libc::c_char)
    {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*CFG).filter_chain);
    } else if 0 as libc::c_int
        == strcmp(showConfigVar, b"output\0" as *const u8 as *const libc::c_char)
    {
        printf(b"%s\0" as *const u8 as *const libc::c_char, (*CFG).output);
        if '\0' as i32
            != *((*CFG).output_arg).offset(0 as libc::c_int as isize) as libc::c_int
        {
            printf(b":%s\0" as *const u8 as *const libc::c_char, (*CFG).output_arg);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else if 0 as libc::c_int
        == strcmp(
            showConfigVar,
            b"syslog_facility\0" as *const u8 as *const libc::c_char,
        )
    {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            snoopy_util_syslog_convertFacilityToStr((*CFG).syslog_facility),
        );
    } else if 0 as libc::c_int
        == strcmp(showConfigVar, b"syslog_ident\0" as *const u8 as *const libc::c_char)
    {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (*CFG).syslog_ident_format,
        );
    } else if 0 as libc::c_int
        == strcmp(showConfigVar, b"syslog_level\0" as *const u8 as *const libc::c_char)
    {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            snoopy_util_syslog_convertLevelToStr((*CFG).syslog_level),
        );
    } else if 0 as libc::c_int
        == strcmp(showConfigVar, b"error_logging\0" as *const u8 as *const libc::c_char)
    {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            if (*CFG).error_logging_enabled == 1 as libc::c_int {
                b"y\0" as *const u8 as *const libc::c_char
            } else {
                b"n\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        fatalErrorValue(
            b"Unknown setting given\0" as *const u8 as *const libc::c_char,
            showConfigVar,
        );
    }
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
