use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn base_len(filename: *const libc::c_char) -> size_t;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrndup(string: *const libc::c_char, n: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn base_name(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut base: *const libc::c_char = last_component(name);
    let mut length: size_t = 0;
    if *base == 0 {
        return xstrndup(name, base_len(name));
    }
    length = base_len(base);
    if *base.offset(length as isize) as libc::c_int == '/' as i32 {
        length = length.wrapping_add(1);
        length;
    }
    if 0 as libc::c_int != 0 {
        let mut p: *mut libc::c_char = xmalloc(
            length.wrapping_add(3 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        *p.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
        *p.offset(1 as libc::c_int as isize) = '/' as i32 as libc::c_char;
        memcpy(
            p.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            base as *const libc::c_void,
            length,
        );
        *p
            .offset(
                length.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        return p;
    }
    return xstrndup(base, length);
}
