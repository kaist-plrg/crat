use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn strlcat(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dsize: size_t,
) -> size_t {
    let mut odst: *const libc::c_char = dst;
    let mut osrc: *const libc::c_char = src;
    let mut n: size_t = dsize;
    let mut dlen: size_t = 0;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0 as libc::c_int as libc::c_ulong
            && *dst as libc::c_int != '\0' as i32)
        {
            break;
        }
        dst = dst.offset(1);
        dst;
    }
    dlen = dst.offset_from(odst) as libc::c_long as size_t;
    n = dsize.wrapping_sub(dlen);
    let fresh1 = n;
    n = n.wrapping_sub(1);
    if fresh1 == 0 as libc::c_int as libc::c_ulong {
        return dlen.wrapping_add(strlen(src));
    }
    while *src as libc::c_int != '\0' as i32 {
        if n != 0 as libc::c_int as libc::c_ulong {
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = *src;
            n = n.wrapping_sub(1);
            n;
        }
        src = src.offset(1);
        src;
    }
    *dst = '\0' as i32 as libc::c_char;
    return dlen.wrapping_add(src.offset_from(osrc) as libc::c_long as libc::c_ulong);
}
