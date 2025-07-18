use ::libc;
extern "C" {
    fn last_component(file: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(file: *const libc::c_char) -> size_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    return xstrndup(base, length);
}
