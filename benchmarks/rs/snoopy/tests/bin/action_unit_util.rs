use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn snoopyTestCli_action_unit_util_syslog(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_util_systemd(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `util`\n\nUsage:\n    snoopy-test unit util SUBUNIT [ARGS]\n\nAvailable subunits:\n    syslog             Run a unit test on src/util/syslog.c\n    systemd            Run a unit test on src/util/systemd.c\n\n    --help,-h          Show this help\n    UNIT --help        Show UNIT's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subunit: *const libc::c_char = 0 as *const libc::c_char;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_unit_util_showHelp();
        fatalError(b"No subunit specified.\0" as *const u8 as *const libc::c_char);
    }
    subunit = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int
        == strcmp(subunit, b"syslog\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_util_syslog(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(subunit, b"systemd\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_util_systemd(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(subunit, b"--help\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subunit, b"-h\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_unit_util_showHelp();
        return 0 as libc::c_int;
    }
    snoopyTestCli_action_unit_util_showHelp();
    fatalErrorValue(b"Unknown subunit\0" as *const u8 as *const libc::c_char, subunit);
    return 127 as libc::c_int;
}
