use ::libc;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub unsafe extern "C" fn hash_hsieh(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut hash: uint32_t = 0 as libc::c_int as uint32_t;
    let mut tmp: uint32_t = 0;
    let mut rem: libc::c_int = 0;
    if key_length <= 0 as libc::c_int as libc::c_ulong || key.is_null() {
        return 0 as libc::c_int as uint32_t;
    }
    rem = (key_length & 3 as libc::c_int as libc::c_ulong) as libc::c_int;
    key_length >>= 2 as libc::c_int;
    while key_length > 0 as libc::c_int as libc::c_ulong {
        hash = (hash as libc::c_uint)
            .wrapping_add(
                ((*(key as *const uint8_t).offset(1 as libc::c_int as isize) as uint32_t)
                    << 8 as libc::c_int)
                    .wrapping_add(
                        *(key as *const uint8_t).offset(0 as libc::c_int as isize)
                            as uint32_t,
                    ),
            ) as uint32_t as uint32_t;
        tmp = ((*(key.offset(2 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int)
            .wrapping_add(
                *(key.offset(2 as libc::c_int as isize) as *const uint8_t)
                    .offset(0 as libc::c_int as isize) as uint32_t,
            ) << 11 as libc::c_int ^ hash;
        hash = hash << 16 as libc::c_int ^ tmp;
        key = key
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as isize,
            );
        hash = (hash as libc::c_uint).wrapping_add(hash >> 11 as libc::c_int) as uint32_t
            as uint32_t;
        key_length = key_length.wrapping_sub(1);
        key_length;
    }
    match rem {
        3 => {
            hash = (hash as libc::c_uint)
                .wrapping_add(
                    ((*(key as *const uint8_t).offset(1 as libc::c_int as isize)
                        as uint32_t) << 8 as libc::c_int)
                        .wrapping_add(
                            *(key as *const uint8_t).offset(0 as libc::c_int as isize)
                                as uint32_t,
                        ),
                ) as uint32_t as uint32_t;
            hash ^= hash << 16 as libc::c_int;
            hash
                ^= (*key
                    .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize)
                    as uint32_t) << 18 as libc::c_int;
            hash = (hash as libc::c_uint).wrapping_add(hash >> 11 as libc::c_int)
                as uint32_t as uint32_t;
        }
        2 => {
            hash = (hash as libc::c_uint)
                .wrapping_add(
                    ((*(key as *const uint8_t).offset(1 as libc::c_int as isize)
                        as uint32_t) << 8 as libc::c_int)
                        .wrapping_add(
                            *(key as *const uint8_t).offset(0 as libc::c_int as isize)
                                as uint32_t,
                        ),
                ) as uint32_t as uint32_t;
            hash ^= hash << 11 as libc::c_int;
            hash = (hash as libc::c_uint).wrapping_add(hash >> 17 as libc::c_int)
                as uint32_t as uint32_t;
        }
        1 => {
            hash = (hash as libc::c_uint)
                .wrapping_add(*key as libc::c_uchar as libc::c_uint) as uint32_t
                as uint32_t;
            hash ^= hash << 10 as libc::c_int;
            hash = (hash as libc::c_uint).wrapping_add(hash >> 1 as libc::c_int)
                as uint32_t as uint32_t;
        }
        _ => {}
    }
    hash ^= hash << 3 as libc::c_int;
    hash = (hash as libc::c_uint).wrapping_add(hash >> 5 as libc::c_int) as uint32_t
        as uint32_t;
    hash ^= hash << 4 as libc::c_int;
    hash = (hash as libc::c_uint).wrapping_add(hash >> 17 as libc::c_int) as uint32_t
        as uint32_t;
    hash ^= hash << 25 as libc::c_int;
    hash = (hash as libc::c_uint).wrapping_add(hash >> 6 as libc::c_int) as uint32_t
        as uint32_t;
    return hash;
}
