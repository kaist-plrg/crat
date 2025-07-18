use ::libc;
extern "C" {
    fn base_len(file: *const libc::c_char) -> size_t;
    fn last_component(file: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn strip_trailing_slashes(mut file: *mut libc::c_char) -> bool {
    let mut base: *mut libc::c_char = last_component(file);
    let mut base_lim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut had_slash: bool = false;
    if *base == 0 {
        base = file;
    }
    base_lim = base.offset(base_len(base) as isize);
    had_slash = *base_lim as libc::c_int != '\0' as i32;
    *base_lim = '\0' as i32 as libc::c_char;
    return had_slash;
}
