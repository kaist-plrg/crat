use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut exit_status: libc::c_int;
    fn fatal_exit();
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub static mut error_hook: Option::<unsafe extern "C" fn() -> ()> = None;
pub unsafe extern "C" fn pax_decode_mode(
    mut mode: mode_t,
    mut string: *mut libc::c_char,
) {
    let fresh0 = string;
    string = string.offset(1);
    *fresh0 = (if mode & 0o400 as libc::c_int as libc::c_uint != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh1 = string;
    string = string.offset(1);
    *fresh1 = (if mode & 0o200 as libc::c_int as libc::c_uint != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh2 = string;
    string = string.offset(1);
    *fresh2 = (if mode & 0o4000 as libc::c_int as libc::c_uint != 0 {
        if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh3 = string;
    string = string.offset(1);
    *fresh3 = (if mode & (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh4 = string;
    string = string.offset(1);
    *fresh4 = (if mode & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh5 = string;
    string = string.offset(1);
    *fresh5 = (if mode & 0o2000 as libc::c_int as libc::c_uint != 0 {
        if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh6 = string;
    string = string.offset(1);
    *fresh6 = (if mode
        & (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh7 = string;
    string = string.offset(1);
    *fresh7 = (if mode
        & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh8 = string;
    string = string.offset(1);
    *fresh8 = (if mode & 0o1000 as libc::c_int as libc::c_uint != 0 {
        if mode
            & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint != 0
        {
            't' as i32
        } else {
            'T' as i32
        }
    } else if mode
        & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *string = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn call_arg_error(
    mut call: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        call,
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn call_arg_fatal(
    mut call: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        call,
    );
    fatal_exit();
}
pub unsafe extern "C" fn call_arg_warn(
    mut call: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Warning: Cannot %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        call,
    );
}
pub unsafe extern "C" fn chmod_error_details(
    mut name: *const libc::c_char,
    mut mode: mode_t,
) {
    let mut e: libc::c_int = *__errno_location();
    let mut buf: [libc::c_char; 10] = [0; 10];
    pax_decode_mode(mode, buf.as_mut_ptr());
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot change mode to %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        buf.as_mut_ptr(),
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn chown_error_details(
    mut name: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
) {
    let mut u: uintmax_t = uid as uintmax_t;
    let mut g: uintmax_t = gid as uintmax_t;
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot change ownership to uid %ju, gid %ju\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        u,
        g,
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn close_error(mut name: *const libc::c_char) {
    call_arg_error(b"close\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn close_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"close\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn exec_fatal(mut name: *const libc::c_char) {
    call_arg_fatal(b"exec\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn link_error(
    mut target: *const libc::c_char,
    mut source: *const libc::c_char,
) {
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot hard link to %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(source),
        quote_n(1 as libc::c_int, target),
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn mkdir_error(mut name: *const libc::c_char) {
    call_arg_error(b"mkdir\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn mkfifo_error(mut name: *const libc::c_char) {
    call_arg_error(b"mkfifo\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn mknod_error(mut name: *const libc::c_char) {
    call_arg_error(b"mknod\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn open_error(mut name: *const libc::c_char) {
    call_arg_error(b"open\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn open_fatal(mut name: *const libc::c_char) {
    call_arg_fatal(b"open\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn open_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"open\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn read_error(mut name: *const libc::c_char) {
    call_arg_error(b"read\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn read_error_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
    mut size: size_t,
) {
    let mut off: intmax_t = offset;
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcngettext(
            0 as *const libc::c_char,
            b"%s: Read error at byte %jd, while reading %zu byte\0" as *const u8
                as *const libc::c_char,
            b"%s: Read error at byte %jd, while reading %zu bytes\0" as *const u8
                as *const libc::c_char,
            size,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        off,
        size,
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn read_warn_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
    mut size: size_t,
) {
    let mut off: intmax_t = offset;
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcngettext(
            0 as *const libc::c_char,
            b"%s: Warning: Read error at byte %jd, while reading %zu byte\0" as *const u8
                as *const libc::c_char,
            b"%s: Warning: Read error at byte %jd, while reading %zu bytes\0"
                as *const u8 as *const libc::c_char,
            size,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        off,
        size,
    );
}
pub unsafe extern "C" fn read_fatal(mut name: *const libc::c_char) {
    call_arg_fatal(b"read\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn read_fatal_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
    mut size: size_t,
) {
    let mut off: intmax_t = offset;
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcngettext(
            0 as *const libc::c_char,
            b"%s: Read error at byte %jd, while reading %zu byte\0" as *const u8
                as *const libc::c_char,
            b"%s: Read error at byte %jd, while reading %zu bytes\0" as *const u8
                as *const libc::c_char,
            size,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        off,
        size,
    );
    fatal_exit();
}
pub unsafe extern "C" fn readlink_error(mut name: *const libc::c_char) {
    call_arg_error(b"readlink\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn readlink_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"readlink\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn rmdir_error(mut name: *const libc::c_char) {
    call_arg_error(b"rmdir\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn savedir_error(mut name: *const libc::c_char) {
    call_arg_error(b"savedir\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn savedir_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"savedir\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn seek_error(mut name: *const libc::c_char) {
    call_arg_error(b"seek\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn seek_error_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
) {
    let mut off: intmax_t = offset;
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot seek to %jd\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        off,
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn seek_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"seek\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn seek_warn_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
) {
    let mut off: intmax_t = offset;
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Warning: Cannot seek to %jd\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        off,
    );
}
pub unsafe extern "C" fn symlink_error(
    mut contents: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut e: libc::c_int = *__errno_location();
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        e,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Cannot create symlink to %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(name),
        quote_n(1 as libc::c_int, contents),
    );
    exit_status = 2 as libc::c_int;
}
pub unsafe extern "C" fn stat_fatal(mut name: *const libc::c_char) {
    call_arg_fatal(b"stat\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn stat_error(mut name: *const libc::c_char) {
    call_arg_error(b"stat\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn stat_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"stat\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn truncate_error(mut name: *const libc::c_char) {
    call_arg_error(b"truncate\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn truncate_warn(mut name: *const libc::c_char) {
    call_arg_warn(b"truncate\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn unlink_error(mut name: *const libc::c_char) {
    call_arg_error(b"unlink\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn utime_error(mut name: *const libc::c_char) {
    call_arg_error(b"utime\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn waitpid_error(mut name: *const libc::c_char) {
    call_arg_error(b"waitpid\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn write_error(mut name: *const libc::c_char) {
    call_arg_error(b"write\0" as *const u8 as *const libc::c_char, name);
}
pub unsafe extern "C" fn write_error_details(
    mut name: *const libc::c_char,
    mut status: size_t,
    mut size: size_t,
) {
    if status == 0 as libc::c_int as libc::c_ulong {
        write_error(name);
    } else {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcngettext(
                0 as *const libc::c_char,
                b"%s: Wrote only %zu of %zu byte\0" as *const u8 as *const libc::c_char,
                b"%s: Wrote only %zu of %zu bytes\0" as *const u8 as *const libc::c_char,
                size,
                5 as libc::c_int,
            ),
            name,
            status,
            size,
        );
        exit_status = 2 as libc::c_int;
    };
}
pub unsafe extern "C" fn chdir_fatal(mut name: *const libc::c_char) {
    call_arg_fatal(b"chdir\0" as *const u8 as *const libc::c_char, name);
}
