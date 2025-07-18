use ::libc;
pub type size_t = libc::c_ulong;
static mut base64_tbl: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &[libc::c_char; 64],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
};
pub unsafe extern "C" fn base64enc(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_void,
    mut count: size_t,
) {
    let mut s: *const libc::c_uchar = src as *const libc::c_uchar;
    let mut d: *mut libc::c_char = dst;
    while count != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut n: libc::c_int = (*s as libc::c_int) << 16 as libc::c_int;
        s = s.offset(1);
        s;
        count = count.wrapping_sub(1);
        count;
        if count != 0 {
            n |= (*s as libc::c_int) << 8 as libc::c_int;
            s = s.offset(1);
            s;
            count = count.wrapping_sub(1);
            count;
            i += 1;
            i;
        }
        if count != 0 {
            n |= *s as libc::c_int;
            s = s.offset(1);
            s;
            count = count.wrapping_sub(1);
            count;
            i += 1;
            i;
        }
        let fresh0 = d;
        d = d.offset(1);
        *fresh0 = base64_tbl[(n >> 18 as libc::c_int & 0x3f as libc::c_int) as usize];
        let fresh1 = d;
        d = d.offset(1);
        *fresh1 = base64_tbl[(n >> 12 as libc::c_int & 0x3f as libc::c_int) as usize];
        let fresh2 = d;
        d = d.offset(1);
        *fresh2 = (if i != 0 {
            base64_tbl[(n >> 6 as libc::c_int & 0x3f as libc::c_int) as usize]
                as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        let fresh3 = d;
        d = d.offset(1);
        *fresh3 = (if i == 2 as libc::c_int {
            base64_tbl[(n & 0x3f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
    }
    *d = 0 as libc::c_int as libc::c_char;
}
