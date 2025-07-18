use ::libc;
extern "C" {
    fn _nettle_camellia_absorb(
        nkeys: libc::c_uint,
        dst: *mut uint64_t,
        subkey: *mut uint64_t,
    );
    static _nettle_camellia_table: camellia_table;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia_table {
    pub sp1110: [uint32_t; 256],
    pub sp0222: [uint32_t; 256],
    pub sp3033: [uint32_t; 256],
    pub sp4404: [uint32_t; 256],
}
pub unsafe extern "C" fn nettle_camellia128_set_encrypt_key(
    mut ctx: *mut camellia128_ctx,
    mut key: *const uint8_t,
) {
    let mut k0: uint64_t = 0;
    let mut k1: uint64_t = 0;
    let mut subkey: [uint64_t; 26] = [0; 26];
    let mut w: uint64_t = 0;
    k0 = (*key.offset(0 as libc::c_int as isize) as uint64_t) << 56 as libc::c_int
        | (*key.offset(1 as libc::c_int as isize) as uint64_t) << 48 as libc::c_int
        | (*key.offset(2 as libc::c_int as isize) as uint64_t) << 40 as libc::c_int
        | (*key.offset(3 as libc::c_int as isize) as uint64_t) << 32 as libc::c_int
        | (*key.offset(4 as libc::c_int as isize) as uint64_t) << 24 as libc::c_int
        | (*key.offset(5 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
        | (*key.offset(6 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
        | *key.offset(7 as libc::c_int as isize) as uint64_t;
    k1 = (*key.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint64_t) << 56 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint64_t) << 48 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint64_t) << 40 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint64_t) << 32 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(4 as libc::c_int as isize)
            as uint64_t) << 24 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(5 as libc::c_int as isize)
            as uint64_t) << 16 as libc::c_int
        | (*key.offset(8 as libc::c_int as isize).offset(6 as libc::c_int as isize)
            as uint64_t) << 8 as libc::c_int
        | *key.offset(8 as libc::c_int as isize).offset(7 as libc::c_int as isize)
            as uint64_t;
    subkey[0 as libc::c_int as usize] = k0;
    subkey[1 as libc::c_int as usize] = k1;
    let mut __rol128_t: uint64_t = k0;
    k0 = k0 << 15 as libc::c_int | k1 >> 64 as libc::c_int - 15 as libc::c_int;
    k1 = k1 << 15 as libc::c_int | __rol128_t >> 64 as libc::c_int - 15 as libc::c_int;
    subkey[4 as libc::c_int as usize] = k0;
    subkey[5 as libc::c_int as usize] = k1;
    let mut __rol128_t_0: uint64_t = k0;
    k0 = k0 << 30 as libc::c_int | k1 >> 64 as libc::c_int - 30 as libc::c_int;
    k1 = k1 << 30 as libc::c_int | __rol128_t_0 >> 64 as libc::c_int - 30 as libc::c_int;
    subkey[10 as libc::c_int as usize] = k0;
    subkey[11 as libc::c_int as usize] = k1;
    let mut __rol128_t_1: uint64_t = k0;
    k0 = k0 << 15 as libc::c_int | k1 >> 64 as libc::c_int - 15 as libc::c_int;
    k1 = k1 << 15 as libc::c_int | __rol128_t_1 >> 64 as libc::c_int - 15 as libc::c_int;
    subkey[13 as libc::c_int as usize] = k1;
    let mut __rol128_t_2: uint64_t = k0;
    k0 = k0 << 17 as libc::c_int | k1 >> 64 as libc::c_int - 17 as libc::c_int;
    k1 = k1 << 17 as libc::c_int | __rol128_t_2 >> 64 as libc::c_int - 17 as libc::c_int;
    subkey[16 as libc::c_int as usize] = k0;
    subkey[17 as libc::c_int as usize] = k1;
    let mut __rol128_t_3: uint64_t = k0;
    k0 = k0 << 17 as libc::c_int | k1 >> 64 as libc::c_int - 17 as libc::c_int;
    k1 = k1 << 17 as libc::c_int | __rol128_t_3 >> 64 as libc::c_int - 17 as libc::c_int;
    subkey[18 as libc::c_int as usize] = k0;
    subkey[19 as libc::c_int as usize] = k1;
    let mut __rol128_t_4: uint64_t = k0;
    k0 = k0 << 17 as libc::c_int | k1 >> 64 as libc::c_int - 17 as libc::c_int;
    k1 = k1 << 17 as libc::c_int | __rol128_t_4 >> 64 as libc::c_int - 17 as libc::c_int;
    subkey[22 as libc::c_int as usize] = k0;
    subkey[23 as libc::c_int as usize] = k1;
    k0 = subkey[0 as libc::c_int as usize];
    w = subkey[1 as libc::c_int as usize];
    let mut __yl: uint32_t = 0;
    let mut __yr: uint32_t = 0;
    let mut __i: uint64_t = (k0 as libc::c_ulonglong
        ^ 0xa09e667f3bcc908b as libc::c_ulonglong) as uint64_t;
    __yl = _nettle_camellia_table
        .sp1110[(__i & 0xff as libc::c_int as libc::c_ulong) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yr = _nettle_camellia_table
        .sp1110[(__i >> 56 as libc::c_int) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yl ^= __yr;
    __yr = __yr << 24 as libc::c_int
        | __yr >> (-(24 as libc::c_int) & 31 as libc::c_int);
    __yr ^= __yl;
    k1 = (__yl as uint64_t) << 32 as libc::c_int | __yr as libc::c_ulong;
    w ^= k1;
    let mut __yl_0: uint32_t = 0;
    let mut __yr_0: uint32_t = 0;
    let mut __i_0: uint64_t = (w as libc::c_ulonglong
        ^ 0xb67ae8584caa73b2 as libc::c_ulonglong) as uint64_t;
    __yl_0 = _nettle_camellia_table
        .sp1110[(__i_0 & 0xff as libc::c_int as libc::c_ulong) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_0 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yr_0 = _nettle_camellia_table
        .sp1110[(__i_0 >> 56 as libc::c_int) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_0 >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_0 >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_0 >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yl_0 ^= __yr_0;
    __yr_0 = __yr_0 << 24 as libc::c_int
        | __yr_0 >> (-(24 as libc::c_int) & 31 as libc::c_int);
    __yr_0 ^= __yl_0;
    k0 = (__yl_0 as uint64_t) << 32 as libc::c_int | __yr_0 as libc::c_ulong;
    let mut __yl_1: uint32_t = 0;
    let mut __yr_1: uint32_t = 0;
    let mut __i_1: uint64_t = (k0 as libc::c_ulonglong
        ^ 0xc6ef372fe94f82be as libc::c_ulonglong) as uint64_t;
    __yl_1 = _nettle_camellia_table
        .sp1110[(__i_1 & 0xff as libc::c_int as libc::c_ulong) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_1 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yr_1 = _nettle_camellia_table
        .sp1110[(__i_1 >> 56 as libc::c_int) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_1 >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_1 >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_1 >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yl_1 ^= __yr_1;
    __yr_1 = __yr_1 << 24 as libc::c_int
        | __yr_1 >> (-(24 as libc::c_int) & 31 as libc::c_int);
    __yr_1 ^= __yl_1;
    w = (__yl_1 as uint64_t) << 32 as libc::c_int | __yr_1 as libc::c_ulong;
    k1 ^= w;
    let mut __yl_2: uint32_t = 0;
    let mut __yr_2: uint32_t = 0;
    let mut __i_2: uint64_t = (k1 as libc::c_ulonglong
        ^ 0x54ff53a5f1d36f1c as libc::c_ulonglong) as uint64_t;
    __yl_2 = _nettle_camellia_table
        .sp1110[(__i_2 & 0xff as libc::c_int as libc::c_ulong) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_2 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yr_2 = _nettle_camellia_table
        .sp1110[(__i_2 >> 56 as libc::c_int) as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_2 >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_2 >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_2 >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_int as usize];
    __yl_2 ^= __yr_2;
    __yr_2 = __yr_2 << 24 as libc::c_int
        | __yr_2 >> (-(24 as libc::c_int) & 31 as libc::c_int);
    __yr_2 ^= __yl_2;
    w = (__yl_2 as uint64_t) << 32 as libc::c_int | __yr_2 as libc::c_ulong;
    k0 ^= w;
    subkey[2 as libc::c_int as usize] = k0;
    subkey[3 as libc::c_int as usize] = k1;
    let mut __rol128_t_5: uint64_t = k0;
    k0 = k0 << 15 as libc::c_int | k1 >> 64 as libc::c_int - 15 as libc::c_int;
    k1 = k1 << 15 as libc::c_int | __rol128_t_5 >> 64 as libc::c_int - 15 as libc::c_int;
    subkey[6 as libc::c_int as usize] = k0;
    subkey[7 as libc::c_int as usize] = k1;
    let mut __rol128_t_6: uint64_t = k0;
    k0 = k0 << 15 as libc::c_int | k1 >> 64 as libc::c_int - 15 as libc::c_int;
    k1 = k1 << 15 as libc::c_int | __rol128_t_6 >> 64 as libc::c_int - 15 as libc::c_int;
    subkey[8 as libc::c_int as usize] = k0;
    subkey[9 as libc::c_int as usize] = k1;
    let mut __rol128_t_7: uint64_t = k0;
    k0 = k0 << 15 as libc::c_int | k1 >> 64 as libc::c_int - 15 as libc::c_int;
    k1 = k1 << 15 as libc::c_int | __rol128_t_7 >> 64 as libc::c_int - 15 as libc::c_int;
    subkey[12 as libc::c_int as usize] = k0;
    let mut __rol128_t_8: uint64_t = k0;
    k0 = k0 << 15 as libc::c_int | k1 >> 64 as libc::c_int - 15 as libc::c_int;
    k1 = k1 << 15 as libc::c_int | __rol128_t_8 >> 64 as libc::c_int - 15 as libc::c_int;
    subkey[14 as libc::c_int as usize] = k0;
    subkey[15 as libc::c_int as usize] = k1;
    let mut __rol128_t_9: uint64_t = k0;
    k0 = k0 << 34 as libc::c_int | k1 >> 64 as libc::c_int - 34 as libc::c_int;
    k1 = k1 << 34 as libc::c_int | __rol128_t_9 >> 64 as libc::c_int - 34 as libc::c_int;
    subkey[20 as libc::c_int as usize] = k0;
    subkey[21 as libc::c_int as usize] = k1;
    let mut __rol128_t_10: uint64_t = k0;
    k0 = k0 << 17 as libc::c_int | k1 >> 64 as libc::c_int - 17 as libc::c_int;
    k1 = k1 << 17 as libc::c_int
        | __rol128_t_10 >> 64 as libc::c_int - 17 as libc::c_int;
    subkey[24 as libc::c_int as usize] = k0;
    subkey[25 as libc::c_int as usize] = k1;
    _nettle_camellia_absorb(
        24 as libc::c_int as libc::c_uint,
        ((*ctx).keys).as_mut_ptr(),
        subkey.as_mut_ptr(),
    );
}
