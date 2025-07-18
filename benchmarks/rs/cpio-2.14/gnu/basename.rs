use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(filename: *const libc::c_char) -> size_t;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
pub unsafe extern "C" fn base_name(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut base: *const libc::c_char = last_component(name);
    let mut length: idx_t = 0;
    let mut dotslash_len: libc::c_int = 0;
    if *base != 0 {
        length = base_len(base) as idx_t;
        length
            += (*base.offset(length as isize) as libc::c_int == '/' as i32)
                as libc::c_int as libc::c_long;
        dotslash_len = if 0 as libc::c_int != 0 as libc::c_int {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        base = name;
        length = base_len(base) as idx_t;
        dotslash_len = 0 as libc::c_int;
    }
    let mut p: *mut libc::c_char = ximalloc(
        dotslash_len as libc::c_long + length + 1 as libc::c_int as libc::c_long,
    ) as *mut libc::c_char;
    if dotslash_len != 0 {
        *p.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
        *p.offset(1 as libc::c_int as isize) = '/' as i32 as libc::c_char;
    }
    memcpy(
        p.offset(dotslash_len as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        length as libc::c_ulong,
    );
    *p
        .offset(
            (dotslash_len as libc::c_long + length) as isize,
        ) = '\0' as i32 as libc::c_char;
    return p;
}
