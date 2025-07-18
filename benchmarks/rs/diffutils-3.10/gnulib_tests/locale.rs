use ::libc;
extern "C" {
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
unsafe extern "C" fn defaulted_getenv(
    mut variable: *const libc::c_char,
) -> *const libc::c_char {
    let mut value: *const libc::c_char = getenv(variable);
    return if !value.is_null() {
        value
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn print_category(
    mut category: libc::c_int,
    mut variable: *const libc::c_char,
) {
    let mut value: *const libc::c_char = defaulted_getenv(variable);
    if *value.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        && *(defaulted_getenv(b"LC_ALL\0" as *const u8 as *const libc::c_char))
            .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        printf(b"%s=%s\n\0" as *const u8 as *const libc::c_char, variable, value);
    } else {
        printf(
            b"%s=\"%s\"\n\0" as *const u8 as *const libc::c_char,
            variable,
            setlocale(category, 0 as *const libc::c_char),
        );
    };
}
unsafe fn main_0() -> libc::c_int {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    printf(
        b"LANG=%s\n\0" as *const u8 as *const libc::c_char,
        defaulted_getenv(b"LANG\0" as *const u8 as *const libc::c_char),
    );
    print_category(0 as libc::c_int, b"LC_CTYPE\0" as *const u8 as *const libc::c_char);
    print_category(
        1 as libc::c_int,
        b"LC_NUMERIC\0" as *const u8 as *const libc::c_char,
    );
    print_category(2 as libc::c_int, b"LC_TIME\0" as *const u8 as *const libc::c_char);
    print_category(
        3 as libc::c_int,
        b"LC_COLLATE\0" as *const u8 as *const libc::c_char,
    );
    print_category(
        4 as libc::c_int,
        b"LC_MONETARY\0" as *const u8 as *const libc::c_char,
    );
    print_category(
        5 as libc::c_int,
        b"LC_MESSAGES\0" as *const u8 as *const libc::c_char,
    );
    print_category(7 as libc::c_int, b"LC_PAPER\0" as *const u8 as *const libc::c_char);
    print_category(8 as libc::c_int, b"LC_NAME\0" as *const u8 as *const libc::c_char);
    print_category(
        9 as libc::c_int,
        b"LC_ADDRESS\0" as *const u8 as *const libc::c_char,
    );
    print_category(
        10 as libc::c_int,
        b"LC_TELEPHONE\0" as *const u8 as *const libc::c_char,
    );
    print_category(
        11 as libc::c_int,
        b"LC_MEASUREMENT\0" as *const u8 as *const libc::c_char,
    );
    print_category(
        12 as libc::c_int,
        b"LC_IDENTIFICATION\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"LC_ALL=%s\n\0" as *const u8 as *const libc::c_char,
        defaulted_getenv(b"LC_ALL\0" as *const u8 as *const libc::c_char),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
