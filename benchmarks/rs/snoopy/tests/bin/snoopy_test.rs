use ::libc;
extern "C" {
    fn snoopyTestCli_action_run(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_stress(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopyTestCli_action_unit(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub static mut g_argc: libc::c_int = 0;
pub static mut g_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub unsafe extern "C" fn snoopyTestCli_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility\n\nUsage:\n    snoopy ACTION [SUBACTION] [ARGS]\n\nAvailable actions:\n    run            Run Snoopy's subsystem\n    stress         Run stress tests\n    unit           Run unit tests\n\n    --help,-h      Show this help\n    ACTION --help  Show ACTION's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut action: *const libc::c_char = 0 as *const libc::c_char;
    g_argc = argc;
    g_argv = argv;
    if argc < 2 as libc::c_int {
        snoopyTestCli_showHelp();
        fatalError(b"No action specified.\0" as *const u8 as *const libc::c_char);
    }
    action = *argv.offset(1 as libc::c_int as isize);
    if 0 as libc::c_int
        == strcmp(action, b"--help\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == strcmp(action, b"-h\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_showHelp();
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(action, b"run\0" as *const u8 as *const libc::c_char) {
        return snoopyTestCli_action_run(
            argc - 2 as libc::c_int,
            &mut *argv.offset(2 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int
        == strcmp(action, b"stress\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_stress(
            argc - 2 as libc::c_int,
            &mut *argv.offset(2 as libc::c_int as isize),
        );
    }
    if 0 as libc::c_int == strcmp(action, b"unit\0" as *const u8 as *const libc::c_char)
    {
        return snoopyTestCli_action_unit(
            argc - 2 as libc::c_int,
            &mut *argv.offset(2 as libc::c_int as isize),
        );
    }
    snoopyTestCli_showHelp();
    fatalErrorValue(b"Unknown action\0" as *const u8 as *const libc::c_char, action);
    return 0;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
