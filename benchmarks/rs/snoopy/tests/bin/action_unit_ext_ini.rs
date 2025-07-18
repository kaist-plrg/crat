use ::libc;
extern "C" {
    fn printSuccess(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snoopy_ini_parse_string(
        string: *const libc::c_char,
        handler: ini_handler,
        user: *mut libc::c_void,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type ini_handler = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub unsafe extern "C" fn snoopyTestCli_action_unit_ext_ini_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `ext-ini`\n\nDescription:\n    Mocks external ini implementation code paths (mainly for the code coverage of parts not used by Snoopy).\n\nUsage:\n    snoopy-test unit ext-ini\n    snoopy-test unit ext-ini --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_ext_ini_parserCallback(
    mut userPtr: *mut libc::c_void,
    mut section: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut errorMessage: *mut *const libc::c_char = userPtr as *mut *const libc::c_char;
    if !(*errorMessage).is_null() {
        return 1 as libc::c_int;
    }
    if 0 as libc::c_int
        != strcmp(section, b"sectionName\0" as *const u8 as *const libc::c_char)
    {
        *errorMessage = b"Unexpected section\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    if 0 as libc::c_int != strcmp(name, b"testKey\0" as *const u8 as *const libc::c_char)
    {
        *errorMessage = b"Unexpected key\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    if 0 as libc::c_int
        != strcmp(value, b"testVal\0" as *const u8 as *const libc::c_char)
    {
        *errorMessage = b"Unexpected value\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_ext_ini(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize);
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_unit_ext_ini_showHelp();
        return 0 as libc::c_int;
    }
    let mut iniContent: *const libc::c_char = b"[sectionName]\ntestKey=testVal\n\0"
        as *const u8 as *const libc::c_char;
    let mut errorMessage: *const libc::c_char = 0 as *const libc::c_char;
    let mut parserStatus: libc::c_int = 0 as libc::c_int;
    parserStatus = snoopy_ini_parse_string(
        iniContent,
        Some(
            snoopyTestCli_action_unit_ext_ini_parserCallback
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        &mut errorMessage as *mut *const libc::c_char as *mut libc::c_void,
    );
    if parserStatus < 0 as libc::c_int {
        fatalError(b"INI parsing failed\0" as *const u8 as *const libc::c_char);
    }
    if !errorMessage.is_null() {
        fatalErrorValue(
            b"INI parsing failure\0" as *const u8 as *const libc::c_char,
            errorMessage,
        );
    }
    printSuccess(
        b"Mocking lib/inih/src/ini.c complete.\0" as *const u8 as *const libc::c_char,
    );
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
