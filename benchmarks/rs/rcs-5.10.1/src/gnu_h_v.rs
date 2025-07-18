use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn generic_warn(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const libc::c_char,
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub help: *const libc::c_char,
    pub tyag: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const hv_version: hv_option_values = 1;
pub const hv_help: hv_option_values = 0;
pub type hv_option_values = libc::c_uint;
pub unsafe extern "C" fn nice_getopt(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut longopts: *const option,
) -> libc::c_int {
    optind = 0 as libc::c_int;
    opterr = 0 as libc::c_int;
    return getopt_long(
        argc,
        argv,
        b"+\0" as *const u8 as *const libc::c_char,
        longopts,
        0 as *mut libc::c_int,
    );
}
pub unsafe extern "C" fn display_version(
    mut prog: *const program,
    mut flags: libc::c_int,
) {
    if 1 as libc::c_int & flags != 0 {
        generic_warn(
            0 as *const libc::c_char,
            b"-V is obsolete; instead, use --version\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(
        b"%s%s\0" as *const u8 as *const libc::c_char,
        (*prog).name,
        b" (GNU RCS) 5.10.1\nCopyright (C) 2010-2022 Thien-Thi Nguyen\nCopyright (C) 1990-1995 Paul Eggert\nCopyright (C) 1982,1988,1989 Walter F. Tichy, Purdue CS\nLicense GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if 2 as libc::c_int & flags != 0 {
        exit(0 as libc::c_int);
    }
}
static mut ok: [option; 3] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: hv_help as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: hv_version as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn check_hv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut prog: *const program,
) {
    if 1 as libc::c_int >= argc {
        return;
    }
    match nice_getopt(argc, argv, ok.as_ptr()) {
        0 => {
            let mut usage: [libc::c_char; 128] = [0; 128];
            let mut nl: libc::c_int = 0;
            snprintf(
                usage.as_mut_ptr(),
                128 as libc::c_int as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*prog).help,
            );
            nl = (strchr(usage.as_mut_ptr(), '\n' as i32))
                .offset_from(usage.as_mut_ptr()) as libc::c_long as libc::c_int;
            usage[nl as usize] = '\0' as i32 as libc::c_char;
            printf(
                b"Usage: %s %s\n\n%s\n%s%s\0" as *const u8 as *const libc::c_char,
                (*prog).name,
                usage.as_mut_ptr(),
                (*prog).desc,
                ((*prog).help).offset(nl as isize),
                b"\nReport bugs to: <bug-rcs@gnu.org>\nRCS home page: <http://www.gnu.org/software/rcs/>\nGeneral help using GNU software: <http://www.gnu.org/gethelp/>\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        1 => {
            display_version(prog, 2 as libc::c_int);
        }
        _ => {}
    };
}
