use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type ptrdiff_t = libc::c_long;
pub unsafe extern "C" fn u8_uctomb_aux(
    mut s: *mut uint8_t,
    mut uc: ucs4_t,
    mut n: ptrdiff_t,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    if uc < 0x80 as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    } else if uc < 0x800 as libc::c_int as libc::c_uint {
        count = 2 as libc::c_int;
    } else if uc < 0x10000 as libc::c_int as libc::c_uint {
        if uc < 0xd800 as libc::c_int as libc::c_uint
            || uc >= 0xe000 as libc::c_int as libc::c_uint
        {
            count = 3 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    } else if uc < 0x110000 as libc::c_int as libc::c_uint {
        count = 4 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    }
    if n < count as libc::c_long {
        return -(2 as libc::c_int);
    }
    let mut current_block_25: u64;
    match count {
        4 => {
            *s
                .offset(
                    3 as libc::c_int as isize,
                ) = (0x80 as libc::c_int as libc::c_uint
                | uc & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            uc = uc >> 6 as libc::c_int;
            uc |= 0x10000 as libc::c_int as libc::c_uint;
            current_block_25 = 9175025349255477743;
        }
        3 => {
            current_block_25 = 9175025349255477743;
        }
        2 => {
            current_block_25 = 2398777646703561453;
        }
        _ => {
            current_block_25 = 224731115979188411;
        }
    }
    match current_block_25 {
        9175025349255477743 => {
            *s
                .offset(
                    2 as libc::c_int as isize,
                ) = (0x80 as libc::c_int as libc::c_uint
                | uc & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            uc = uc >> 6 as libc::c_int;
            uc |= 0x800 as libc::c_int as libc::c_uint;
            current_block_25 = 2398777646703561453;
        }
        _ => {}
    }
    match current_block_25 {
        2398777646703561453 => {
            *s
                .offset(
                    1 as libc::c_int as isize,
                ) = (0x80 as libc::c_int as libc::c_uint
                | uc & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            uc = uc >> 6 as libc::c_int;
            uc |= 0xc0 as libc::c_int as libc::c_uint;
            *s.offset(0 as libc::c_int as isize) = uc as uint8_t;
        }
        _ => {}
    }
    return count;
}
