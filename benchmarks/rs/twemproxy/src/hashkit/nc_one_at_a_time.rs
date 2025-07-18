use ::libc;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn hash_one_at_a_time(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut ptr: *const libc::c_char = key;
    let mut value: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        let fresh0 = key_length;
        key_length = key_length.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        let mut val: uint32_t = *fresh1 as uint32_t;
        value = (value as libc::c_uint).wrapping_add(val) as uint32_t as uint32_t;
        value = (value as libc::c_uint).wrapping_add(value << 10 as libc::c_int)
            as uint32_t as uint32_t;
        value ^= value >> 6 as libc::c_int;
    }
    value = (value as libc::c_uint).wrapping_add(value << 3 as libc::c_int) as uint32_t
        as uint32_t;
    value ^= value >> 11 as libc::c_int;
    value = (value as libc::c_uint).wrapping_add(value << 15 as libc::c_int) as uint32_t
        as uint32_t;
    return value;
}
