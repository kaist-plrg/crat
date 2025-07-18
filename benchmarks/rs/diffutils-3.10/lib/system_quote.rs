use ::libc;
extern "C" {
    fn abort() -> !;
    fn shell_quote_length(string: *const libc::c_char) -> size_t;
    fn shell_quote_copy(
        p: *mut libc::c_char,
        string: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn shell_quote(string: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type system_command_interpreter = libc::c_uint;
pub const SCI_POSIX_SH: system_command_interpreter = 1;
pub const SCI_SYSTEM: system_command_interpreter = 0;
pub unsafe extern "C" fn system_quote_length(
    mut interpreter: system_command_interpreter,
    mut string: *const libc::c_char,
) -> size_t {
    match interpreter as libc::c_uint {
        0 | 1 => return shell_quote_length(string),
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn system_quote_copy(
    mut p: *mut libc::c_char,
    mut interpreter: system_command_interpreter,
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    match interpreter as libc::c_uint {
        0 | 1 => return shell_quote_copy(p, string),
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn system_quote(
    mut interpreter: system_command_interpreter,
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    match interpreter as libc::c_uint {
        0 | 1 => return shell_quote(string),
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn system_quote_argv(
    mut interpreter: system_command_interpreter,
    mut argv: *const *mut libc::c_char,
) -> *mut libc::c_char {
    if !(*argv).is_null() {
        let mut argp: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
        let mut length: size_t = 0;
        let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        length = 0 as libc::c_int as size_t;
        argp = argv;
        loop {
            length = (length as libc::c_ulong)
                .wrapping_add(
                    (system_quote_length(interpreter, *argp))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            argp = argp.offset(1);
            argp;
            if (*argp).is_null() {
                break;
            }
        }
        command = (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            == 1 as libc::c_int as libc::c_ulong
        {
            xmalloc(length)
        } else {
            xnmalloc(length, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        }) as *mut libc::c_char;
        p = command;
        argp = argv;
        loop {
            p = system_quote_copy(p, interpreter, *argp);
            argp = argp.offset(1);
            argp;
            if (*argp).is_null() {
                break;
            }
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = ' ' as i32 as libc::c_char;
        }
        *p = '\0' as i32 as libc::c_char;
        return command;
    } else {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char)
    };
}
