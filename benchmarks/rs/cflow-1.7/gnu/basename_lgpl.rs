use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn last_component(
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut base: *const libc::c_char = name.offset(0 as libc::c_int as isize);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut last_was_slash: bool = 0 as libc::c_int != 0;
    while *base as libc::c_int == '/' as i32 {
        base = base.offset(1);
        base;
    }
    p = base;
    while *p != 0 {
        if *p as libc::c_int == '/' as i32 {
            last_was_slash = 1 as libc::c_int != 0;
        } else if last_was_slash {
            base = p;
            last_was_slash = 0 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
    }
    return base as *mut libc::c_char;
}
pub unsafe extern "C" fn base_len(mut name: *const libc::c_char) -> size_t {
    let mut len: size_t = 0;
    let mut prefix_len: size_t = 0 as libc::c_int as size_t;
    len = strlen(name);
    while (1 as libc::c_int as libc::c_ulong) < len
        && *name.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
        len;
    }
    if 0 as libc::c_int != 0 && len == 1 as libc::c_int as libc::c_ulong
        && *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *name.offset(2 as libc::c_int as isize) == 0
    {
        return 2 as libc::c_int as size_t;
    }
    if 0 as libc::c_int != 0 && prefix_len != 0 && len == prefix_len
        && *name.offset(prefix_len as isize) as libc::c_int == '/' as i32
    {
        return prefix_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    return len;
}
