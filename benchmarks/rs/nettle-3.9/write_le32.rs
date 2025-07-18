use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub unsafe extern "C" fn _nettle_write_le32(
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint32_t,
) {
    let mut i: size_t = 0;
    let mut words: size_t = 0;
    let mut leftover: libc::c_uint = 0;
    words = length.wrapping_div(4 as libc::c_int as libc::c_ulong);
    leftover = length.wrapping_rem(4 as libc::c_int as libc::c_ulong) as libc::c_uint;
    i = 0 as libc::c_int as size_t;
    while i < words {
        *dst
            .offset(
                3 as libc::c_int as isize,
            ) = (*src.offset(i as isize) >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(
                2 as libc::c_int as isize,
            ) = (*src.offset(i as isize) >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = (*src.offset(i as isize) >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = (*src.offset(i as isize) & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(4 as libc::c_int as isize);
    }
    if leftover != 0 {
        let mut word: uint32_t = 0;
        word = *src.offset(i as isize);
        loop {
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = (word & 0xff as libc::c_int as libc::c_uint) as uint8_t;
            word >>= 8 as libc::c_int;
            leftover = leftover.wrapping_sub(1);
            if !(leftover != 0) {
                break;
            }
        }
    }
}
