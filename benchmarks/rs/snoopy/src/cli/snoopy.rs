use ::libc;
extern "C" {
    fn snoopy_cli_action_conf() -> libc::c_int;
    fn snoopy_cli_action_disable() -> libc::c_int;
    fn snoopy_cli_action_enable() -> libc::c_int;
    fn snoopy_cli_action_status() -> libc::c_int;
    fn snoopy_cli_action_version() -> libc::c_int;
    fn fatalError(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn showAbout() {
    let mut aboutContent: *mut libc::c_char = b"Snoopy is a small library that logs all program executions on your Linux/BSD system (a.k.a. Snoopy Logger).\nThe command line utility that you've just used to display this message is Snoopy's CLI management tool.\n\nMore information about Snoopy is available at the following URL:\n    https://github.com/a2o/snoopy/\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, aboutContent);
}
pub unsafe extern "C" fn showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy CLI management tool usage:\n    snoopy ACTION [ARGS]\n\nAvailable actions:\n    conf           Show configuration\n    disable        Remove libsnoopy.so from /etc/ld.so.preload\n    enable         Add libsnoopy.so to /etc/ld.so.preload\n    status         Detect whether Snoopy is already enabled and/or loaded\n\n    version        Show Snoopy version\n    about          Show general information\n    help           Show this help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        showHelp();
        fatalError(b"No action specified.\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"status\0" as *const u8 as *const libc::c_char,
        )
    {
        return snoopy_cli_action_status()
    } else if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"conf\0" as *const u8 as *const libc::c_char,
        )
    {
        return snoopy_cli_action_conf()
    } else if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"disable\0" as *const u8 as *const libc::c_char,
        )
    {
        return snoopy_cli_action_disable()
    } else if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"enable\0" as *const u8 as *const libc::c_char,
        )
    {
        return snoopy_cli_action_enable()
    } else if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"version\0" as *const u8 as *const libc::c_char,
        )
    {
        return snoopy_cli_action_version()
    } else if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"about\0" as *const u8 as *const libc::c_char,
        )
    {
        showAbout();
        return 0 as libc::c_int;
    } else if 0 as libc::c_int
        == strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"help\0" as *const u8 as *const libc::c_char,
        )
    {
        showHelp();
        return 0 as libc::c_int;
    } else {
        showHelp();
        fatalErrorValue(
            b"Unknown action\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
    }
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
