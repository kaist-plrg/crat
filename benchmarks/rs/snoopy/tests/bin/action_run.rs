use ::libc;
extern "C" {
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn snoopyTestCli_action_run_configfile(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_run_datasource(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_run_filter(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_run_filterchain(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_run_messageformat(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_run_output(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_run_everything() -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run`\n\nUsage:\n    snoopy run SUBSYSTEM [ARGS]\n\nAvailable subsystems:\n    configfile,cf      Run a configfile (.ini) parser\n    datasource,ds      Run a data source\n    filter,f           Run a filter\n    filterchain,fc     Run a filter chain (as if it would be configured in snoopy.ini)\n    messageformat,mf   Run the message formatter\n    output,o           Run an output\n\n    everything         Runs every subsystem, as much as possible (for Valgrind)\n\n    help,h             Show this help\n    SUBSYSTEM help     Show SUBSYSTEM's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_showHelp();
        fatalError(b"No subsystem specified.\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"configfile\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"cf\0" as *const u8 as *const libc::c_char,
            )
    {
        return snoopyTestCli_action_run_configfile(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"datasource\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"ds\0" as *const u8 as *const libc::c_char,
            )
    {
        return snoopyTestCli_action_run_datasource(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"filter\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"f\0" as *const u8 as *const libc::c_char,
            )
    {
        return snoopyTestCli_action_run_filter(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"filterchain\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"fc\0" as *const u8 as *const libc::c_char,
            )
    {
        return snoopyTestCli_action_run_filterchain(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"messageformat\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"mf\0" as *const u8 as *const libc::c_char,
            )
    {
        return snoopyTestCli_action_run_messageformat(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"output\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"o\0" as *const u8 as *const libc::c_char,
            )
    {
        return snoopyTestCli_action_run_output(
            argc - 1 as libc::c_int,
            &mut *argv.offset(1 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"everything\0" as *const u8 as *const libc::c_char,
        )
    {
        return snoopyTestCli_action_run_everything();
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"help\0" as *const u8 as *const libc::c_char,
        )
        || 0 as libc::c_int
            == strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"h\0" as *const u8 as *const libc::c_char,
            )
    {
        snoopyTestCli_action_run_showHelp();
        return 0 as libc::c_int;
    }
    snoopyTestCli_action_run_showHelp();
    fatalErrorValue(
        b"Unknown subsystem\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    return 127 as libc::c_int;
}
