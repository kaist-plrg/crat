use ::libc;
extern "C" {
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub unsafe extern "C" fn hard_locale(mut category: libc::c_int) -> bool {
    let mut hard: bool = 1 as libc::c_int != 0;
    let mut p: *const libc::c_char = setlocale(category, 0 as *const libc::c_char);
    if !p.is_null() {
        if 2 as libc::c_int <= 2 as libc::c_int {
            if strcmp(p, b"C\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(p, b"POSIX\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                hard = 0 as libc::c_int != 0;
            }
        } else {
            let mut locale: *mut libc::c_char = strdup(p);
            if !locale.is_null() {
                p = setlocale(category, b"C\0" as *const u8 as *const libc::c_char);
                if !p.is_null() && strcmp(p, locale) == 0 as libc::c_int
                    || {
                        p = setlocale(
                            category,
                            b"POSIX\0" as *const u8 as *const libc::c_char,
                        );
                        !p.is_null() && strcmp(p, locale) == 0 as libc::c_int
                    }
                {
                    hard = 0 as libc::c_int != 0;
                }
                setlocale(category, locale);
                free(locale as *mut libc::c_void);
            }
        }
    }
    return hard;
}
