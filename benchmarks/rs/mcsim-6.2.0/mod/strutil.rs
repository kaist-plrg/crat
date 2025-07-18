use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn MyStrcmp(
    mut sz1: *const libc::c_char,
    mut sz2: *const libc::c_char,
) -> libc::c_int {
    if sz1.is_null() {
        if !sz2.is_null() { return -(1 as libc::c_int) } else { return 0 as libc::c_int }
    } else if !sz2.is_null() {
        return strcmp(sz1, sz2)
    } else {
        return 1 as libc::c_int
    };
}
