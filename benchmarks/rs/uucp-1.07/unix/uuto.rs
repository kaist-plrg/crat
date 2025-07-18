use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn zsysdep_uuto(
    mut zdest: *const libc::c_char,
    mut zlocalname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut zexclam: *const libc::c_char = 0 as *const libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    zexclam = strrchr(zdest, '!' as i32);
    if zexclam.is_null() {
        return 0 as *mut libc::c_char;
    }
    zto = zbufalc(
        (zexclam.offset_from(zdest) as libc::c_long as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_add(strlen(zexclam))
            .wrapping_add(strlen(zlocalname)),
    );
    memcpy(
        zto as *mut libc::c_void,
        zdest as *const libc::c_void,
        zexclam.offset_from(zdest) as libc::c_long as size_t,
    );
    sprintf(
        zto.offset(zexclam.offset_from(zdest) as libc::c_long as isize),
        b"!~/receive/%s/%s/\0" as *const u8 as *const libc::c_char,
        zexclam.offset(1 as libc::c_int as isize),
        zlocalname,
    );
    return zto;
}
