use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn concatenated_filename(
    mut directory: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(directory, b".\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        result = malloc(
            (strlen(filename))
                .wrapping_add(
                    if !suffix.is_null() {
                        strlen(suffix)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    },
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if result.is_null() {
            return 0 as *mut libc::c_char;
        }
        p = result;
    } else {
        let mut directory_len: size_t = strlen(directory);
        let mut need_slash: libc::c_int = (directory_len
            > 0 as libc::c_int as libc::c_ulong
            && !(*directory
                .offset(
                    directory_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '/' as i32)) as libc::c_int;
        result = malloc(
            directory_len
                .wrapping_add(need_slash as libc::c_ulong)
                .wrapping_add(strlen(filename))
                .wrapping_add(
                    if !suffix.is_null() {
                        strlen(suffix)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    },
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if result.is_null() {
            return 0 as *mut libc::c_char;
        }
        memcpy(
            result as *mut libc::c_void,
            directory as *const libc::c_void,
            directory_len,
        );
        p = result.offset(directory_len as isize);
        if need_slash != 0 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '/' as i32 as libc::c_char;
        }
    }
    p = stpcpy(p, filename);
    if !suffix.is_null() {
        stpcpy(p, suffix);
    }
    return result;
}
