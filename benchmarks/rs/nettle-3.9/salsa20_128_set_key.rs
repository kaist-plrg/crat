use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
pub unsafe extern "C" fn nettle_salsa20_128_set_key(
    mut ctx: *mut salsa20_ctx,
    mut key: *const uint8_t,
) {
    (*ctx)
        .input[1 as libc::c_int
        as usize] = (*key
        .offset(0 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx).input[11 as libc::c_int as usize] = (*ctx).input[1 as libc::c_int as usize];
    (*ctx)
        .input[2 as libc::c_int
        as usize] = (*key
        .offset(4 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx).input[12 as libc::c_int as usize] = (*ctx).input[2 as libc::c_int as usize];
    (*ctx)
        .input[3 as libc::c_int
        as usize] = (*key
        .offset(8 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx).input[13 as libc::c_int as usize] = (*ctx).input[3 as libc::c_int as usize];
    (*ctx)
        .input[4 as libc::c_int
        as usize] = (*key
        .offset(12 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(12 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(12 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(12 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t;
    (*ctx).input[14 as libc::c_int as usize] = (*ctx).input[4 as libc::c_int as usize];
    (*ctx).input[0 as libc::c_int as usize] = 0x61707865 as libc::c_int as uint32_t;
    (*ctx).input[5 as libc::c_int as usize] = 0x3120646e as libc::c_int as uint32_t;
    (*ctx).input[10 as libc::c_int as usize] = 0x79622d36 as libc::c_int as uint32_t;
    (*ctx).input[15 as libc::c_int as usize] = 0x6b206574 as libc::c_int as uint32_t;
}
