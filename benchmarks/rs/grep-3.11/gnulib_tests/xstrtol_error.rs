use ::libc;
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
unsafe extern "C" fn xstrtol_error(
    mut err: strtol_error,
    mut opt_idx: libc::c_int,
    mut c: libc::c_char,
    mut long_options: *const option,
    mut arg: *const libc::c_char,
    mut exit_status: libc::c_int,
) {
    let mut hyphens: *const libc::c_char = b"--\0" as *const u8 as *const libc::c_char;
    let mut msgid: *const libc::c_char = 0 as *const libc::c_char;
    let mut option: *const libc::c_char = 0 as *const libc::c_char;
    let mut option_buffer: [libc::c_char; 2] = [0; 2];
    match err as libc::c_uint {
        4 => {
            msgid = b"invalid %s%s argument '%s'\0" as *const u8 as *const libc::c_char;
        }
        2 | 3 => {
            msgid = b"invalid suffix in %s%s argument '%s'\0" as *const u8
                as *const libc::c_char;
        }
        1 => {
            msgid = b"%s%s argument '%s' too large\0" as *const u8
                as *const libc::c_char;
        }
        _ => {
            abort();
        }
    }
    if opt_idx < 0 as libc::c_int {
        hyphens = hyphens.offset(-(opt_idx as isize));
        option_buffer[0 as libc::c_int as usize] = c;
        option_buffer[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        option = option_buffer.as_mut_ptr();
    } else {
        option = (*long_options.offset(opt_idx as isize)).name;
    }
    error(
        exit_status,
        0 as libc::c_int,
        dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
        hyphens,
        option,
        arg,
    );
}
pub unsafe extern "C" fn xstrtol_fatal(
    mut err: strtol_error,
    mut opt_idx: libc::c_int,
    mut c: libc::c_char,
    mut long_options: *const option,
    mut arg: *const libc::c_char,
) {
    xstrtol_error(err, opt_idx, c, long_options, arg, exit_failure);
    abort();
}
