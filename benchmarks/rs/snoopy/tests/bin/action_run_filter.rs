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
    fn snoopy_filterregistry_getCount() -> libc::c_int;
    fn snoopy_filterregistry_doesNameExist(
        filterName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_filterregistry_getName(filterId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_filterregistry_callById(
        filterId: libc::c_int,
        filterArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_filterregistry_callByName(
        filterName: *const libc::c_char,
        filterArg: *const libc::c_char,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter_showList() {
    printf(b"Available filters:\n\0" as *const u8 as *const libc::c_char);
    let mut fCount: libc::c_int = snoopy_filterregistry_getCount();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < fCount {
        printf(
            b"    %s\n\0" as *const u8 as *const libc::c_char,
            snoopy_filterregistry_getName(i),
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `filter`\n\nUsage:\n    snoopy-test run filter FILTER [FILTER_ARGS]\n    snoopy-test run filter --all\n    snoopy-test run filter --list\n    snoopy-test run filter --help\n\nResult:\n    Prints the result of a called filter as a \"PASS\" or a \"DROP\" to stdout.\n    Sets the exit status to 0 or PASS or 1 for DROP.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
    snoopyTestCli_action_run_filter_showList();
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterName: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterResult: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_filter_showHelp();
        fatalError(
            b"Missing argument: filter name, or --all, or --list, or --help\0"
                as *const u8 as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int == strcmp(arg1, b"--all\0" as *const u8 as *const libc::c_char) {
        snoopyTestCli_action_run_filter_all();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_filter_showHelp();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--list\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_run_filter_showList();
        return 0 as libc::c_int;
    }
    filterName = arg1;
    if argc > 1 as libc::c_int {
        filterArg = *argv.offset(1 as libc::c_int as isize);
    } else {
        filterArg = b"\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int == snoopy_filterregistry_doesNameExist(filterName) {
        snoopyTestCli_action_run_filter_showHelp();
        fatalErrorValue(
            b"Invalid filter name given\0" as *const u8 as *const libc::c_char,
            filterName,
        );
    }
    filterResult = snoopy_filterregistry_callByName(filterName, filterArg);
    snoopy_entrypoint_test_cli_exit();
    if 1 as libc::c_int == filterResult {
        printf(b"PASS\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    } else {
        printf(b"DROP\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter_all() {
    let mut itemName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemArgs: *const libc::c_char = 0 as *const libc::c_char;
    let mut itemResult: libc::c_int = 0;
    let mut fCount: libc::c_int = 0;
    fCount = snoopy_filterregistry_getCount();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < fCount {
        itemName = snoopy_filterregistry_getName(i);
        printf(b"Filter %19s: \0" as *const u8 as *const libc::c_char, itemName);
        if strcmp(itemName, b"exclude_spawns_of\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"asdf,bsdf\0" as *const u8 as *const libc::c_char;
        } else if strcmp(itemName, b"exclude_uid\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"0\0" as *const u8 as *const libc::c_char;
        } else if strcmp(itemName, b"only_root\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"\0" as *const u8 as *const libc::c_char;
        } else if strcmp(itemName, b"only_uid\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            itemArgs = b"0\0" as *const u8 as *const libc::c_char;
        } else {
            itemArgs = b"\0" as *const u8 as *const libc::c_char;
        }
        itemResult = snoopy_filterregistry_callById(i, itemArgs);
        if 1 as libc::c_int == itemResult {
            printf(b"PASS\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"DROP\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
