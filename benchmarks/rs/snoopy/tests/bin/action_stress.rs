use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn snoopyTestCli_action_stress_threads(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_stress_threadsexec(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `stress`\n\nUsage:\n    snoopy stress SUBSYSTEM [ARGS]\n\nAvailable subsystems:\n    threads,t          Stress internal threading implementation\n    threadsexec,te     Stress threading implementation by including execution of an external command\n\n    help,h             Show this help\n    SUBSYSTEM help     Show SUBSYSTEM's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subsystem: *const libc::c_char = 0 as *const libc::c_char;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_stress_showHelp();
        fatalError(b"No subsystem specified.\0" as *const u8 as *const libc::c_char);
    }
    subsystem = *argv.offset(0 as libc::c_int as isize);
    if 0 as libc::c_int
        == strcmp(subsystem, b"help\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subsystem, b"h\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_stress_showHelp();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int
        == strcmp(subsystem, b"threads\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subsystem, b"t\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_stress_threads(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(subsystem, b"threadsexec\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(subsystem, b"te\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_stress_threadsexec(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    snoopyTestCli_action_stress_showHelp();
    fatalErrorValue(
        b"Unknown subsystem\0" as *const u8 as *const libc::c_char,
        subsystem,
    );
    return 127 as libc::c_int;
}
