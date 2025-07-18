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
    fn snoopy_filtering_check_chain(chain: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filterchain_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `filterchain`\n\nUsage:\n    snoopy-test run filterchain \"FILTER_CHAIN\"\n    snoopy-test run filterchain --help\n\nDescription:\n    Runs MESSAGE through a specified FILTER_CHAIN, with filters acting on the data taken from the current process.\n    Filter chain specification format is described in the comments of snoopy.ini.\n\nResult:\n    Prints the result of a filter chain as a \"PASS\" or a \"DROP\" to stdout.\n    Sets the exit status to 0 or PASS or 1 for DROP.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filterchain(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterChain: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterResult: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_filterchain_showHelp();
        fatalError(
            b"Missing argument: filter chain specification, or --help\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_filterchain_showHelp();
        return 0 as libc::c_int;
    }
    filterChain = arg1;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_filterchain_showHelp();
        fatalError(
            b"Missing argument: filter chain specification\0" as *const u8
                as *const libc::c_char,
        );
    }
    filterResult = snoopy_filtering_check_chain(filterChain);
    snoopy_entrypoint_test_cli_exit();
    if 1 as libc::c_int == filterResult {
        printf(b"PASS\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    } else {
        printf(b"DROP\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    };
}
