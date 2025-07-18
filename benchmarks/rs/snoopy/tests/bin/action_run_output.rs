use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_outputregistry_getCount() -> libc::c_int;
    fn snoopy_outputregistry_doesNameExist(
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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output_showList() {
    printf(b"Available outputs:\n\0" as *const u8 as *const libc::c_char);
    let mut oCount: libc::c_int = snoopy_outputregistry_getCount();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < oCount {
        printf(
            b"    %s\n\0" as *const u8 as *const libc::c_char,
            snoopy_outputregistry_getName(i),
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `output`\n\nUsage:\n    snoopy-test run output \"LOG MESSAGE\" OUTPUT [OUTPUT_ARGS]\n    snoopy-test run output --all\n    snoopy-test run output --list\n    snoopy-test run output --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
    snoopyTestCli_action_run_output_showList();
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut message: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputName: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut retVal: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_output_showHelp();
        fatalError(
            b"Missing argument: log message, or --all or --list\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int == strcmp(arg1, b"--all\0" as *const u8 as *const libc::c_char) {
        snoopyTestCli_action_run_output_all();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_output_showHelp();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--list\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_output_showList();
        return 0 as libc::c_int;
    }
    message = arg1;
    if argc < 2 as libc::c_int {
        snoopyTestCli_action_run_output_showHelp();
        fatalError(
            b"Missing argument: output name\0" as *const u8 as *const libc::c_char,
        );
    }
    outputName = *argv.offset(1 as libc::c_int as isize);
    if argc > 2 as libc::c_int {
        outputArg = *argv.offset(2 as libc::c_int as isize);
    } else {
        outputArg = b"\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int == snoopy_outputregistry_doesNameExist(outputName) {
        snoopyTestCli_action_run_output_showHelp();
        fatalErrorValue(
            b"Invalid output name given\0" as *const u8 as *const libc::c_char,
            outputName,
        );
    }
    retVal = snoopy_outputregistry_callByName(outputName, message, outputArg);
    if retVal < 0 as libc::c_int {
        fatalError(b"Output failure\0" as *const u8 as *const libc::c_char);
    }
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output_all() {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemArgs: *const libc::c_char = 0 as *const libc::c_char;
    let mut itemResult: libc::c_int = 0;
    let mut iCount: libc::c_int = 0;
    message = malloc(16383 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    snprintf(
        message,
        16383 as libc::c_int as libc::c_ulong,
        b"Snoopy output debugging\0" as *const u8 as *const libc::c_char,
    );
    iCount = snoopy_outputregistry_getCount();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iCount {
        itemName = snoopy_outputregistry_getName(i);
        printf(b"Output %19s: \0" as *const u8 as *const libc::c_char, itemName);
        if strcmp(itemName, b"file\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"./fileoutput.out\0" as *const u8 as *const libc::c_char;
        } else if strcmp(itemName, b"socket\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"/dev/log\0" as *const u8 as *const libc::c_char;
        } else {
            itemArgs = b"\0" as *const u8 as *const libc::c_char;
        }
        itemResult = snoopy_outputregistry_callById(i, message, itemArgs);
        printf(
            b"%d chars transmitted. (output arg:%s)\n\0" as *const u8
                as *const libc::c_char,
            itemResult,
            itemArgs,
        );
        if strcmp(itemName, b"file\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            unlink(itemArgs);
        }
        i += 1;
        i;
    }
    free(message as *mut libc::c_void);
}
