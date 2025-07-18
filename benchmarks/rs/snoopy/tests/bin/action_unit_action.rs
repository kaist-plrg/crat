use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn snoopyTestCli_action_unit_action_log_message_dispatch(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit_action_log_syscall_exec(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `action`\n\nUsage:\n    snoopy-test unit action SUBUNIT [ARGS]\n\nAvailable subunits:\n    log-message-dispatch,lmd   Run a unit test on src/action/log-message-dispatch.c\n    log-syscall-exec,lse       Run a unit test on src/action/log-syscall-exec.c\n\n    --help,-h          Show this help\n    UNIT --help        Show UNIT's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subunit: *const libc::c_char = 0 as *const libc::c_char;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_unit_action_showHelp();
        fatalError(b"No subunit specified.\0" as *const u8 as *const libc::c_char);
    }
    subunit = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int
        == strcmp(subunit, b"log-message-dispatch\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subunit, b"lmd\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_action_log_message_dispatch(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(subunit, b"log-syscall-exec\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subunit, b"lse\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit_action_log_syscall_exec(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(subunit, b"--help\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subunit, b"-h\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_unit_action_showHelp();
        return 0 as libc::c_int;
    }
    snoopyTestCli_action_unit_action_showHelp();
    fatalErrorValue(b"Unknown subunit\0" as *const u8 as *const libc::c_char, subunit);
    return 127 as libc::c_int;
}
