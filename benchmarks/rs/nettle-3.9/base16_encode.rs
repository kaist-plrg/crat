use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
static mut hex_digits: [uint8_t; 16] = unsafe {
    *::std::mem::transmute::<&[u8; 16], &[uint8_t; 16]>(b"0123456789abcdef")
};
pub unsafe extern "C" fn nettle_base16_encode_single(
    mut dst: *mut libc::c_char,
    mut src: uint8_t,
) {
    *dst
        .offset(
            0 as libc::c_int as isize,
        ) = hex_digits[(src as libc::c_int / 0x10 as libc::c_int & 0xf as libc::c_int)
        as usize] as libc::c_char;
    *dst
        .offset(
            1 as libc::c_int as isize,
        ) = hex_digits[(src as libc::c_int & 0xf as libc::c_int) as usize]
        as libc::c_char;
}
pub unsafe extern "C" fn nettle_base16_encode_update(
    mut dst: *mut libc::c_char,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        nettle_base16_encode_single(dst, *src.offset(i as isize));
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(2 as libc::c_int as isize);
    }
}
