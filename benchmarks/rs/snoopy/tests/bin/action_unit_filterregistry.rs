use ::libc;
extern "C" {
    fn printSuccess(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn snoopy_filterregistry_getCount() -> libc::c_int;
    fn snoopy_filterregistry_doesIdExist(filterId: libc::c_int) -> libc::c_int;
    fn snoopy_filterregistry_getIdFromName(
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
pub unsafe extern "C" fn snoopyTestCli_action_unit_filterregistry_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `filterregistry`\n\nDescription:\n    Mocks filterregistry implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit filterregistry\n    snoopy-test unit filterregistry --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_filterregistry(
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
        snoopyTestCli_action_unit_filterregistry_showHelp();
        return 0 as libc::c_int;
    }
    let mut filterName: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterCount: libc::c_int = 0 as libc::c_int;
    let mut filterIdPreset: libc::c_int = 0 as libc::c_int;
    let mut filterIdReceived: libc::c_int = 0 as libc::c_int;
    filterCount = snoopy_filterregistry_getCount();
    if filterCount < 1 as libc::c_int {
        fatalError(
            b"No filters available, filter count is 0\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(
        b"[DEBUG] Number of filters available: %d\n\0" as *const u8
            as *const libc::c_char,
        filterCount,
    );
    if snoopy_filterregistry_doesIdExist(filterIdPreset) != 1 as libc::c_int {
        fatalError(
            b"Filter with a preset ID (0) does not exist\0" as *const u8
                as *const libc::c_char,
        );
    }
    filterName = snoopy_filterregistry_getName(filterIdPreset);
    filterIdReceived = snoopy_filterregistry_getIdFromName(filterName);
    if filterIdPreset != filterIdReceived {
        fatalError(
            b"Filter ID returned when searching by name differs from the initially used ID to find that same filter\0"
                as *const u8 as *const libc::c_char,
        );
    }
    filterName = b"noop\0" as *const u8 as *const libc::c_char;
    filterIdReceived = snoopy_filterregistry_getIdFromName(filterName);
    snoopy_filterregistry_callById(
        filterIdReceived,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if -(1 as libc::c_int)
        != snoopy_filterregistry_callById(
            -(1 as libc::c_int),
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalError(
            b"Filter ID -1 unexpectedly exists\0" as *const u8 as *const libc::c_char,
        );
    }
    if -(1 as libc::c_int)
        != snoopy_filterregistry_callByName(
            b"fakeFilterNameThatShouldNeverExist\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalError(
            b"Filter with an unexpected name actually exists\0" as *const u8
                as *const libc::c_char,
        );
    }
    printSuccess(
        b"Mocking src/filterregistry.c complete.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
