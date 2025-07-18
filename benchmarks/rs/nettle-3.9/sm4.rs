use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm4_ctx {
    pub rkey: [uint32_t; 32],
}
static mut fk: [uint32_t; 4] = [
    0xa3b1bac6 as libc::c_uint,
    0x56aa3350 as libc::c_int as uint32_t,
    0x677d9197 as libc::c_int as uint32_t,
    0xb27022dc as libc::c_uint,
];
static mut ck: [uint32_t; 32] = [
    0x70e15 as libc::c_int as uint32_t,
    0x1c232a31 as libc::c_int as uint32_t,
    0x383f464d as libc::c_int as uint32_t,
    0x545b6269 as libc::c_int as uint32_t,
    0x70777e85 as libc::c_int as uint32_t,
    0x8c939aa1 as libc::c_uint,
    0xa8afb6bd as libc::c_uint,
    0xc4cbd2d9 as libc::c_uint,
    0xe0e7eef5 as libc::c_uint,
    0xfc030a11 as libc::c_uint,
    0x181f262d as libc::c_int as uint32_t,
    0x343b4249 as libc::c_int as uint32_t,
    0x50575e65 as libc::c_int as uint32_t,
    0x6c737a81 as libc::c_int as uint32_t,
    0x888f969d as libc::c_uint,
    0xa4abb2b9 as libc::c_uint,
    0xc0c7ced5 as libc::c_uint,
    0xdce3eaf1 as libc::c_uint,
    0xf8ff060d as libc::c_uint,
    0x141b2229 as libc::c_int as uint32_t,
    0x30373e45 as libc::c_int as uint32_t,
    0x4c535a61 as libc::c_int as uint32_t,
    0x686f767d as libc::c_int as uint32_t,
    0x848b9299 as libc::c_uint,
    0xa0a7aeb5 as libc::c_uint,
    0xbcc3cad1 as libc::c_uint,
    0xd8dfe6ed as libc::c_uint,
    0xf4fb0209 as libc::c_uint,
    0x10171e25 as libc::c_int as uint32_t,
    0x2c333a41 as libc::c_int as uint32_t,
    0x484f565d as libc::c_int as uint32_t,
    0x646b7279 as libc::c_int as uint32_t,
];
static mut sbox: [uint8_t; 256] = [
    0xd6 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0xe9 as libc::c_int as uint8_t,
    0xfe as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0xe1 as libc::c_int as uint8_t,
    0x3d as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0xbe as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0xef as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0x33 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0xed as libc::c_int as uint8_t,
    0xcf as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0xe4 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xe6 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x4f as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0x6b as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xeb as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0x4b as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x5e as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0x3b as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0x27 as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0xe7 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0xea as libc::c_int as uint8_t,
    0xbf as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0xd2 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xce as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0xe0 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x5d as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xe3 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0xe2 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0x6f as libc::c_int as uint8_t,
    0xd5 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0xfd as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0x6d as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0x5b as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0xdd as libc::c_int as uint8_t,
    0xbc as libc::c_int as uint8_t,
    0x7f as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x1f as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0x7b as libc::c_int as uint8_t,
    0xbd as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x74 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xe5 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0x7e as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xec as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0xdc as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0xee as libc::c_int as uint8_t,
    0x5f as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0xd7 as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
];
unsafe extern "C" fn sm4_t_non_lin_sub(mut x: uint32_t) -> uint32_t {
    let mut out: uint32_t = 0;
    out = sbox[(x & 0xff as libc::c_int as libc::c_uint) as usize] as uint32_t;
    out
        |= (sbox[(x >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize]
            as uint32_t) << 8 as libc::c_int;
    out
        |= (sbox[(x >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize]
            as uint32_t) << 16 as libc::c_int;
    out
        |= (sbox[(x >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize]
            as uint32_t) << 24 as libc::c_int;
    return out;
}
unsafe extern "C" fn sm4_key_lin_sub(mut x: uint32_t) -> uint32_t {
    return x ^ (x << 13 as libc::c_int | x >> (-(13 as libc::c_int) & 31 as libc::c_int))
        ^ (x << 23 as libc::c_int | x >> (-(23 as libc::c_int) & 31 as libc::c_int));
}
unsafe extern "C" fn sm4_enc_lin_sub(mut x: uint32_t) -> uint32_t {
    return x ^ (x << 2 as libc::c_int | x >> (-(2 as libc::c_int) & 31 as libc::c_int))
        ^ (x << 10 as libc::c_int | x >> (-(10 as libc::c_int) & 31 as libc::c_int))
        ^ (x << 18 as libc::c_int | x >> (-(18 as libc::c_int) & 31 as libc::c_int))
        ^ (x << 24 as libc::c_int | x >> (-(24 as libc::c_int) & 31 as libc::c_int));
}
unsafe extern "C" fn sm4_key_sub(mut x: uint32_t) -> uint32_t {
    return sm4_key_lin_sub(sm4_t_non_lin_sub(x));
}
unsafe extern "C" fn sm4_enc_sub(mut x: uint32_t) -> uint32_t {
    return sm4_enc_lin_sub(sm4_t_non_lin_sub(x));
}
unsafe extern "C" fn sm4_round(
    mut x0: uint32_t,
    mut x1: uint32_t,
    mut x2: uint32_t,
    mut x3: uint32_t,
    mut rk: uint32_t,
) -> uint32_t {
    return x0 ^ sm4_enc_sub(x1 ^ x2 ^ x3 ^ rk);
}
unsafe extern "C" fn sm4_set_key(
    mut ctx: *mut sm4_ctx,
    mut key: *const uint8_t,
    mut encrypt: libc::c_int,
) {
    let mut rk0: uint32_t = 0;
    let mut rk1: uint32_t = 0;
    let mut rk2: uint32_t = 0;
    let mut rk3: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    rk0 = ((*key.offset(0 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint32_t) << 24 as libc::c_int
        | (*key.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(0 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(0 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint32_t) ^ fk[0 as libc::c_int as usize];
    rk1 = ((*key.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint32_t) << 24 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(4 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint32_t) ^ fk[1 as libc::c_int as usize];
    rk2 = ((*key.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint32_t) << 24 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(8 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint32_t) ^ fk[2 as libc::c_int as usize];
    rk3 = ((*key.offset(12 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint32_t) << 24 as libc::c_int
        | (*key.offset(12 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*key.offset(12 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *key.offset(12 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint32_t) ^ fk[3 as libc::c_int as usize];
    i = 0 as libc::c_int as libc::c_uint;
    while i < 32 as libc::c_int as libc::c_uint {
        rk0
            ^= sm4_key_sub(
                rk1 ^ rk2 ^ rk3
                    ^ ck[i.wrapping_add(0 as libc::c_int as libc::c_uint) as usize],
            );
        rk1
            ^= sm4_key_sub(
                rk2 ^ rk3 ^ rk0
                    ^ ck[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize],
            );
        rk2
            ^= sm4_key_sub(
                rk3 ^ rk0 ^ rk1
                    ^ ck[i.wrapping_add(2 as libc::c_int as libc::c_uint) as usize],
            );
        rk3
            ^= sm4_key_sub(
                rk0 ^ rk1 ^ rk2
                    ^ ck[i.wrapping_add(3 as libc::c_int as libc::c_uint) as usize],
            );
        if encrypt != 0 {
            (*ctx).rkey[i.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] = rk0;
            (*ctx).rkey[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] = rk1;
            (*ctx).rkey[i.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] = rk2;
            (*ctx).rkey[i.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] = rk3;
        } else {
            (*ctx)
                .rkey[((31 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                .wrapping_sub(i) as usize] = rk0;
            (*ctx)
                .rkey[((31 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                .wrapping_sub(i) as usize] = rk1;
            (*ctx)
                .rkey[((31 as libc::c_int - 2 as libc::c_int) as libc::c_uint)
                .wrapping_sub(i) as usize] = rk2;
            (*ctx)
                .rkey[((31 as libc::c_int - 3 as libc::c_int) as libc::c_uint)
                .wrapping_sub(i) as usize] = rk3;
        }
        i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
pub unsafe extern "C" fn nettle_sm4_set_encrypt_key(
    mut ctx: *mut sm4_ctx,
    mut key: *const uint8_t,
) {
    sm4_set_key(ctx, key, 1 as libc::c_int);
}
pub unsafe extern "C" fn nettle_sm4_set_decrypt_key(
    mut ctx: *mut sm4_ctx,
    mut key: *const uint8_t,
) {
    sm4_set_key(ctx, key, 0 as libc::c_int);
}
pub unsafe extern "C" fn nettle_sm4_crypt(
    mut context: *const sm4_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut rk: *const uint32_t = ((*context).rkey).as_ptr();
    if length.wrapping_rem(16 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(length % SM4_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
            b"sm4.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void nettle_sm4_crypt(const struct sm4_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2616: {
        if length.wrapping_rem(16 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!(length % SM4_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
                b"sm4.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void nettle_sm4_crypt(const struct sm4_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut x0: uint32_t = 0;
        let mut x1: uint32_t = 0;
        let mut x2: uint32_t = 0;
        let mut x3: uint32_t = 0;
        let mut i: libc::c_uint = 0;
        x0 = (*src
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*src
                .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*src
                .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *src
                .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(3 as libc::c_int as isize) as uint32_t;
        x1 = (*src
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*src
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*src
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *src
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(3 as libc::c_int as isize) as uint32_t;
        x2 = (*src
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*src
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*src
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *src
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(3 as libc::c_int as isize) as uint32_t;
        x3 = (*src
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*src
                .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*src
                .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *src
                .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                .offset(3 as libc::c_int as isize) as uint32_t;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 32 as libc::c_int as libc::c_uint {
            x0 = sm4_round(
                x0,
                x1,
                x2,
                x3,
                *rk.offset(i.wrapping_add(0 as libc::c_int as libc::c_uint) as isize),
            );
            x1 = sm4_round(
                x1,
                x2,
                x3,
                x0,
                *rk.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
            );
            x2 = sm4_round(
                x2,
                x3,
                x0,
                x1,
                *rk.offset(i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize),
            );
            x3 = sm4_round(
                x3,
                x0,
                x1,
                x2,
                *rk.offset(i.wrapping_add(3 as libc::c_int as libc::c_uint) as isize),
            );
            i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
        }
        *dst
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (x3 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (x2 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (x1 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (x0 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        src = src.offset(16 as libc::c_int as isize);
        dst = dst.offset(16 as libc::c_int as isize);
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
}
