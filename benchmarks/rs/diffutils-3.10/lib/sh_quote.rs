use ::libc;
extern "C" {
    pub type quoting_options;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    fn quotearg_buffer(
        buffer: *mut libc::c_char,
        buffersize: size_t,
        arg: *const libc::c_char,
        argsize: size_t,
        o: *const quoting_options,
    ) -> size_t;
    fn quotearg_alloc(
        arg: *const libc::c_char,
        argsize: size_t,
        o: *const quoting_options,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
static mut sh_quoting_options: *mut quoting_options = 0 as *const quoting_options
    as *mut quoting_options;
unsafe extern "C" fn init_sh_quoting_options() {
    sh_quoting_options = clone_quoting_options(0 as *mut quoting_options);
    set_quoting_style(sh_quoting_options, shell_quoting_style);
}
pub unsafe extern "C" fn shell_quote_length(mut string: *const libc::c_char) -> size_t {
    if sh_quoting_options.is_null() {
        init_sh_quoting_options();
    }
    return quotearg_buffer(
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        string,
        strlen(string),
        sh_quoting_options,
    );
}
pub unsafe extern "C" fn shell_quote_copy(
    mut p: *mut libc::c_char,
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    if sh_quoting_options.is_null() {
        init_sh_quoting_options();
    }
    return p
        .offset(
            quotearg_buffer(
                p,
                -(1 as libc::c_int) as size_t,
                string,
                strlen(string),
                sh_quoting_options,
            ) as isize,
        );
}
pub unsafe extern "C" fn shell_quote(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    if sh_quoting_options.is_null() {
        init_sh_quoting_options();
    }
    return quotearg_alloc(string, strlen(string), sh_quoting_options);
}
pub unsafe extern "C" fn shell_quote_argv(
    mut argv: *const *const libc::c_char,
) -> *mut libc::c_char {
    if !(*argv).is_null() {
        let mut argp: *const *const libc::c_char = 0 as *const *const libc::c_char;
        let mut length: size_t = 0;
        let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        length = 0 as libc::c_int as size_t;
        argp = argv;
        loop {
            length = (length as libc::c_ulong)
                .wrapping_add(
                    (shell_quote_length(*argp))
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
            p = shell_quote_copy(p, *argp);
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
