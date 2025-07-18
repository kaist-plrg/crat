use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    static mut g_argv: *mut *mut libc::c_char;
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_error_handler(errorMsg: *const libc::c_char);
    fn snoopy_datasourceregistry_getCount() -> libc::c_int;
    fn snoopy_datasourceregistry_doesNameExist(
        datasourceName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasourceregistry_getName(datasourceId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_datasourceregistry_callById(
        datasourceId: libc::c_int,
        result: *mut libc::c_char,
        datasourceArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasourceregistry_callByName(
        datasourceName: *const libc::c_char,
        result: *mut libc::c_char,
        datasourceArg: *const libc::c_char,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource_showList() {
    printf(b"Available datasources:\n\0" as *const u8 as *const libc::c_char);
    let mut dCount: libc::c_int = snoopy_datasourceregistry_getCount();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < dCount {
        printf(
            b"    %s\n\0" as *const u8 as *const libc::c_char,
            snoopy_datasourceregistry_getName(i),
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `datasource`\n\nUsage:\n    snoopy-test run datasource DATASOURCE [ARGS]\n    snoopy-test run datasource --all    # Runs all datasources\n    snoopy-test run datasource --help   # Shows this help message\n    snoopy-test run datasource --list   # Lists all available datasources\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
    snoopyTestCli_action_run_datasource_showList();
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut datasourceName: *const libc::c_char = 0 as *const libc::c_char;
    let mut datasourceArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut datasourceResult: [libc::c_char; 2048] = [0; 2048];
    let mut retVal: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_datasource_showHelp();
        fatalError(
            b"Missing argument: `datasource name` or `--list`\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int == strcmp(arg1, b"--all\0" as *const u8 as *const libc::c_char) {
        snoopyTestCli_action_run_datasource_all();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_datasource_showHelp();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--list\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_datasource_showList();
        return 0 as libc::c_int;
    }
    datasourceName = arg1;
    if 0 as libc::c_int == snoopy_datasourceregistry_doesNameExist(datasourceName) {
        snoopyTestCli_action_run_datasource_showHelp();
        fatalError(
            b"Invalid datasource name given\0" as *const u8 as *const libc::c_char,
        );
    }
    if argc >= 2 as libc::c_int {
        datasourceArg = *argv.offset(1 as libc::c_int as isize);
    } else {
        datasourceArg = b"\0" as *const u8 as *const libc::c_char;
    }
    retVal = snoopy_datasourceregistry_callByName(
        datasourceName,
        datasourceResult.as_mut_ptr(),
        datasourceArg,
    );
    if retVal < 0 as libc::c_int {
        fatalErrorValue(
            b"Datasource failed\0" as *const u8 as *const libc::c_char,
            datasourceResult.as_mut_ptr(),
        );
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, datasourceResult.as_mut_ptr());
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource_all() {
    let mut itemName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemArgs: *const libc::c_char = 0 as *const libc::c_char;
    let mut itemResult: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemResultSize: libc::c_int = 0;
    let mut dCount: libc::c_int = 0;
    itemResult = malloc((2048 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    dCount = snoopy_datasourceregistry_getCount();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < dCount {
        itemName = snoopy_datasourceregistry_getName(i);
        printf(b"Datasource %15s: \0" as *const u8 as *const libc::c_char, itemName);
        if strcmp(itemName, b"env\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"HOME\0" as *const u8 as *const libc::c_char;
        } else if strcmp(
            itemName,
            b"snoopy_literal\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            itemArgs = b"somestring\0" as *const u8 as *const libc::c_char;
        } else {
            itemArgs = b"\0" as *const u8 as *const libc::c_char;
        }
        itemResultSize = snoopy_datasourceregistry_callById(i, itemResult, itemArgs);
        if itemResultSize > 2048 as libc::c_int {
            snoopy_error_handler(
                b"Maximum data source message size exceeded\0" as *const u8
                    as *const libc::c_char,
            );
        }
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, itemResult);
        i += 1;
        i;
    }
    free(itemResult as *mut libc::c_void);
}
