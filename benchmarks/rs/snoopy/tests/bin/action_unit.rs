use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn snoopyTestCli_action_unit_action(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_error(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_ext_ini(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_filterregistry(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_outputregistry(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_util(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit`\n\nUsage:\n    snoopy-test unit UNIT [ARGS]\n\nAvailable units:\n    action,a           Run a unit test on action/*.c\n    error,e            Run a unit test on src/error.c\n    ext-ini,ei         Run a unit test on lib/inih/src/ini.c\n    filterregistry,fr  Run a unit test on src/filterregistry.c\n    outputregistry,or  Run a unit test on src/outputregistry.c\n    util,u             Run a unit test on src/util/*.c\n\n    --help,-h          Show this help\n    UNIT --help        Show UNIT's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut unit: *const libc::c_char = 0 as *const libc::c_char;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_unit_showHelp();
        fatalError(b"No unit specified.\0" as *const u8 as *const libc::c_char);
    }
    unit = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int == strcmp(unit, b"action\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"a\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_action(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int == strcmp(unit, b"error\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"e\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_error(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int == strcmp(unit, b"ext-ini\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"ei\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_ext_ini(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(unit, b"filterregistry\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"fr\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_filterregistry(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(unit, b"outputregistry\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"or\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_outputregistry(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int == strcmp(unit, b"util\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"u\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_util(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int == strcmp(unit, b"--help\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(unit, b"-h\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_unit_showHelp();
        return 0 as libc::c_int;
    }
    snoopyTestCli_action_unit_showHelp();
    fatalErrorValue(b"Unknown unit\0" as *const u8 as *const libc::c_char, unit);
    return 127 as libc::c_int;
}
