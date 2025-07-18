use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut environ: *mut *mut libc::c_char;
}
static mut argv0: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut argv_lth: libc::c_int = 0;
pub unsafe extern "C" fn initproctitle(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut envp: *mut *mut libc::c_char = environ;
    i = 0 as libc::c_int;
    while !(*envp.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    environ = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((i + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if environ.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while !(*envp.offset(i as isize)).is_null() {
        let ref mut fresh0 = *environ.offset(i as isize);
        *fresh0 = strdup(*envp.offset(i as isize));
        if (*fresh0).is_null() {
            return;
        }
        i += 1;
        i;
    }
    let ref mut fresh1 = *environ.offset(i as isize);
    *fresh1 = 0 as *mut libc::c_char;
    argv0 = argv;
    if i > 0 as libc::c_int {
        argv_lth = (*envp.offset((i - 1 as libc::c_int) as isize))
            .offset(strlen(*envp.offset((i - 1 as libc::c_int) as isize)) as isize)
            .offset_from(*argv0.offset(0 as libc::c_int as isize)) as libc::c_long
            as libc::c_int;
    } else {
        argv_lth = (*argv0.offset((argc - 1 as libc::c_int) as isize))
            .offset(strlen(*argv0.offset((argc - 1 as libc::c_int) as isize)) as isize)
            .offset_from(*argv0.offset(0 as libc::c_int as isize)) as libc::c_long
            as libc::c_int;
    };
}
pub unsafe extern "C" fn getproctitle(
    mut procbuffer: *mut *mut libc::c_char,
) -> libc::c_int {
    if argv0.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        *argv0.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        '\0' as i32,
        argv_lth as libc::c_ulong,
    );
    let ref mut fresh2 = *argv0.offset(1 as libc::c_int as isize);
    *fresh2 = 0 as *mut libc::c_char;
    *procbuffer = *argv0.offset(0 as libc::c_int as isize);
    return argv_lth;
}
pub unsafe extern "C" fn setproctitle(
    mut prog: *const libc::c_char,
    mut txt: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    if argv0.is_null() {
        return;
    }
    if (strlen(prog))
        .wrapping_add(strlen(txt))
        .wrapping_add(5 as libc::c_int as libc::c_ulong)
        > 2048 as libc::c_int as libc::c_ulong
    {
        return;
    }
    sprintf(
        buf.as_mut_ptr(),
        b"%s -- %s\0" as *const u8 as *const libc::c_char,
        prog,
        txt,
    );
    i = strlen(buf.as_mut_ptr()) as libc::c_int;
    if i > argv_lth - 2 as libc::c_int {
        i = argv_lth - 2 as libc::c_int;
        buf[i as usize] = '\0' as i32 as libc::c_char;
    }
    memset(
        *argv0.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        '\0' as i32,
        argv_lth as libc::c_ulong,
    );
    strcpy(*argv0.offset(0 as libc::c_int as isize), buf.as_mut_ptr());
    let ref mut fresh3 = *argv0.offset(1 as libc::c_int as isize);
    *fresh3 = 0 as *mut libc::c_char;
}
