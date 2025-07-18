use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static _nettle_blowfish_initial_ctx: blowfish_ctx;
    fn _nettle_blowfish_encround(
        ctx: *const blowfish_ctx,
        ret_xl: *mut uint32_t,
        ret_xr: *mut uint32_t,
    );
    fn nettle_base64_encode_init(ctx: *mut base64_encode_ctx);
    fn nettle_base64_encode_update(
        ctx: *mut base64_encode_ctx,
        dst: *mut libc::c_char,
        length: size_t,
        src: *const uint8_t,
    ) -> size_t;
    fn nettle_base64_encode_final(
        ctx: *mut base64_encode_ctx,
        dst: *mut libc::c_char,
    ) -> size_t;
    fn nettle_base64_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blowfish_ctx {
    pub s: [[uint32_t; 256]; 4],
    pub p: [uint32_t; 18],
}
pub type bf_key = [uint32_t; 18];
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub salt: [uint32_t; 4],
    pub output: [uint32_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ctx: blowfish_ctx,
    pub expanded_key: bf_key,
    pub binary: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const libc::c_char,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
static mut radix64_decode_table: [libc::c_schar; 256] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    54 as libc::c_int as libc::c_schar,
    55 as libc::c_int as libc::c_schar,
    56 as libc::c_int as libc::c_schar,
    57 as libc::c_int as libc::c_schar,
    58 as libc::c_int as libc::c_schar,
    59 as libc::c_int as libc::c_schar,
    60 as libc::c_int as libc::c_schar,
    61 as libc::c_int as libc::c_schar,
    62 as libc::c_int as libc::c_schar,
    63 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    45 as libc::c_int as libc::c_schar,
    46 as libc::c_int as libc::c_schar,
    47 as libc::c_int as libc::c_schar,
    48 as libc::c_int as libc::c_schar,
    49 as libc::c_int as libc::c_schar,
    50 as libc::c_int as libc::c_schar,
    51 as libc::c_int as libc::c_schar,
    52 as libc::c_int as libc::c_schar,
    53 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
];
static mut radix64_encode_table: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &[libc::c_char; 64],
    >(b"./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
};
pub unsafe extern "C" fn nettle_blowfish_bcrypt_verify(
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut lenhashed: size_t,
    mut hashed: *const uint8_t,
) -> libc::c_int {
    let mut newhash: [uint8_t; 61] = [0; 61];
    return (nettle_blowfish_bcrypt_hash(
        newhash.as_mut_ptr(),
        lenkey,
        key,
        lenhashed,
        hashed,
        -(1 as libc::c_int),
        0 as *const uint8_t,
    ) != 0
        && strcmp(
            newhash.as_mut_ptr() as *const libc::c_char,
            hashed as *const libc::c_char,
        ) == 0) as libc::c_int;
}
unsafe extern "C" fn encode_radix64(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut src: *const uint8_t,
) -> *mut libc::c_char {
    let mut ctx: base64_encode_ctx = base64_encode_ctx {
        alphabet: 0 as *const libc::c_char,
        word: 0,
        bits: 0,
    };
    nettle_base64_encode_init(&mut ctx);
    ctx.alphabet = radix64_encode_table.as_ptr();
    dst = dst.offset(nettle_base64_encode_update(&mut ctx, dst, len, src) as isize);
    dst = dst.offset(nettle_base64_encode_final(&mut ctx, dst) as isize);
    dst = dst.offset(-1);
    *dst = '\0' as i32 as libc::c_char;
    return dst;
}
static mut magic_w: [uint32_t; 6] = [
    0x4f727068 as libc::c_int as uint32_t,
    0x65616e42 as libc::c_int as uint32_t,
    0x65686f6c as libc::c_int as uint32_t,
    0x64657253 as libc::c_int as uint32_t,
    0x63727944 as libc::c_int as uint32_t,
    0x6f756274 as libc::c_int as uint32_t,
];
unsafe extern "C" fn bswap32_if_le(mut x: *mut uint32_t, mut n: libc::c_uint) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        *x.offset(i as isize) = (*x.offset(i as isize)).swap_bytes();
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn set_xkey(
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut expanded: *mut uint32_t,
    mut initial: *mut uint32_t,
    mut bug: libc::c_uint,
    mut safety: uint32_t,
) {
    let mut ptr: *const uint8_t = key;
    let mut n: size_t = lenkey;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut sign: uint32_t = 0;
    let mut diff: uint32_t = 0;
    let mut tmp: [uint32_t; 2] = [0; 2];
    diff = 0 as libc::c_int as uint32_t;
    sign = diff;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (16 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
        tmp[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
        tmp[0 as libc::c_int as usize] = tmp[1 as libc::c_int as usize];
        j = 0 as libc::c_int as libc::c_uint;
        while j < 4 as libc::c_int as libc::c_uint {
            tmp[0 as libc::c_int as usize] <<= 8 as libc::c_int;
            tmp[0 as libc::c_int as usize] |= *ptr as libc::c_uint;
            tmp[1 as libc::c_int as usize] <<= 8 as libc::c_int;
            tmp[1 as libc::c_int as usize] |= *ptr as libc::c_schar as libc::c_uint;
            if j != 0 {
                sign
                    |= tmp[1 as libc::c_int as usize]
                        & 0x80 as libc::c_int as libc::c_uint;
            }
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if fresh0 != 0 {
                ptr = ptr.offset(1);
                ptr;
            } else {
                ptr = key;
                n = lenkey;
            }
            j = j.wrapping_add(1);
            j;
        }
        diff |= tmp[0 as libc::c_int as usize] ^ tmp[1 as libc::c_int as usize];
        *expanded.offset(i as isize) = tmp[bug as usize];
        *initial
            .offset(
                i as isize,
            ) = _nettle_blowfish_initial_ctx.p[i as usize] ^ tmp[bug as usize];
        i = i.wrapping_add(1);
        i;
    }
    diff |= diff >> 16 as libc::c_int;
    diff &= 0xffff as libc::c_int as libc::c_uint;
    diff = (diff as libc::c_uint).wrapping_add(0xffff as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    sign <<= 9 as libc::c_int;
    sign &= !diff & safety;
    let ref mut fresh1 = *initial.offset(0 as libc::c_int as isize);
    *fresh1 ^= sign;
}
unsafe extern "C" fn ibcrypt(
    mut dst: *mut uint8_t,
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut lenscheme: size_t,
    mut scheme: *const uint8_t,
    mut minlog2rounds: libc::c_int,
    mut log2rounds: libc::c_int,
    mut salt: *const uint8_t,
) -> libc::c_int {
    let mut data: C2RustUnnamed_0 = C2RustUnnamed_0 {
        ctx: blowfish_ctx {
            s: [[0; 256]; 4],
            p: [0; 18],
        },
        expanded_key: [0; 18],
        binary: C2RustUnnamed { salt: [0; 4] },
    };
    let mut psalt: [uint8_t; 16] = [0; 16];
    let mut L: uint32_t = 0;
    let mut R: uint32_t = 0;
    let mut ptr: *mut uint32_t = 0 as *mut uint32_t;
    let mut count: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut cscheme: libc::c_uint = 0;
    let mut bug: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut safety: uint32_t = 0 as libc::c_int as uint32_t;
    if lenscheme < 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if lenscheme >= 3 as libc::c_int as libc::c_ulong
        && {
            let fresh2 = scheme;
            scheme = scheme.offset(1);
            *fresh2 as libc::c_int != '$' as i32
        }
    {
        return 0 as libc::c_int;
    }
    let fresh3 = scheme;
    scheme = scheme.offset(1);
    if *fresh3 as libc::c_int != '2' as i32 {
        return 0 as libc::c_int;
    }
    let fresh4 = scheme;
    scheme = scheme.offset(1);
    cscheme = *fresh4 as libc::c_uint;
    match cscheme {
        97 => {
            safety = 0x10000 as libc::c_int as uint32_t;
        }
        120 => {
            bug = 1 as libc::c_int as libc::c_uint;
        }
        98 | 121 => {}
        _ => return 0 as libc::c_int,
    }
    if lenscheme >= 4 as libc::c_int as libc::c_ulong {
        let fresh5 = scheme;
        scheme = scheme.offset(1);
        if *fresh5 as libc::c_int != '$' as i32 {
            return 0 as libc::c_int;
        }
        if lenscheme >= 6 as libc::c_int as libc::c_ulong {
            if log2rounds < 0 as libc::c_int {
                let fresh6 = scheme;
                scheme = scheme.offset(1);
                let mut c: libc::c_uint = (*fresh6 as libc::c_int - '0' as i32)
                    as libc::c_uint;
                if c > 9 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int;
                }
                log2rounds = c.wrapping_mul(10 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let fresh7 = scheme;
                scheme = scheme.offset(1);
                c = (*fresh7 as libc::c_int - '0' as i32) as libc::c_uint;
                if c > 9 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int;
                }
                log2rounds = (log2rounds as libc::c_uint).wrapping_add(c) as libc::c_int
                    as libc::c_int;
            } else {
                scheme = scheme.offset(2 as libc::c_int as isize);
            }
            if lenscheme >= 7 as libc::c_int as libc::c_ulong
                && {
                    let fresh8 = scheme;
                    scheme = scheme.offset(1);
                    *fresh8 as libc::c_int != '$' as i32
                }
            {
                return 0 as libc::c_int;
            }
            if lenscheme
                >= (7 as libc::c_int
                    + (16 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int)
                        / 6 as libc::c_int) as libc::c_ulong && salt.is_null()
            {
                let mut ctx: base64_decode_ctx = base64_decode_ctx {
                    table: 0 as *const libc::c_schar,
                    word: 0,
                    bits: 0,
                    padding: 0,
                };
                let mut saltlen: size_t = 16 as libc::c_int as size_t;
                nettle_base64_decode_init(&mut ctx);
                ctx.table = radix64_decode_table.as_ptr();
                if nettle_base64_decode_update(
                    &mut ctx,
                    &mut saltlen,
                    (data.binary.salt).as_mut_ptr() as *mut uint8_t,
                    ((16 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int)
                        / 6 as libc::c_int) as size_t,
                    scheme as *const libc::c_char,
                ) == 0 || saltlen != 16 as libc::c_int as libc::c_ulong
                {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    if !salt.is_null() {
        memcpy(
            (data.binary.salt).as_mut_ptr() as *mut libc::c_void,
            salt as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
    } else if lenscheme
        < (7 as libc::c_int
            + (16 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int)
                / 6 as libc::c_int) as libc::c_ulong
    {
        return 0 as libc::c_int
    }
    memcpy(
        psalt.as_mut_ptr() as *mut libc::c_void,
        (data.binary.salt).as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    bswap32_if_le((data.binary.salt).as_mut_ptr(), 4 as libc::c_int as libc::c_uint);
    if log2rounds < minlog2rounds || log2rounds > 31 as libc::c_int {
        return 0 as libc::c_int;
    }
    count = (1 as libc::c_int as uint32_t) << log2rounds;
    set_xkey(
        lenkey,
        key,
        (data.expanded_key).as_mut_ptr(),
        (data.ctx.p).as_mut_ptr(),
        bug,
        safety,
    );
    memcpy(
        (data.ctx.s).as_mut_ptr() as *mut libc::c_void,
        (_nettle_blowfish_initial_ctx.s).as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[[uint32_t; 256]; 4]>() as libc::c_ulong,
    );
    R = 0 as libc::c_int as uint32_t;
    L = R;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int + 2 as libc::c_int {
        L ^= data.binary.salt[(i & 2 as libc::c_int) as usize];
        R ^= data.binary.salt[((i & 2 as libc::c_int) + 1 as libc::c_int) as usize];
        _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
        data.ctx.p[i as usize] = L;
        data.ctx.p[(i + 1 as libc::c_int) as usize] = R;
        i += 2 as libc::c_int;
    }
    ptr = (data.ctx.s[0 as libc::c_int as usize]).as_mut_ptr();
    loop {
        ptr = ptr.offset(4 as libc::c_int as isize);
        L
            ^= data
                .binary
                .salt[(16 as libc::c_int + 2 as libc::c_int & 3 as libc::c_int)
                as usize];
        R
            ^= data
                .binary
                .salt[(16 as libc::c_int + 3 as libc::c_int & 3 as libc::c_int)
                as usize];
        _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
        *ptr.offset(-(4 as libc::c_int as isize)) = L;
        *ptr.offset(-(3 as libc::c_int as isize)) = R;
        L
            ^= data
                .binary
                .salt[(16 as libc::c_int + 4 as libc::c_int & 3 as libc::c_int)
                as usize];
        R
            ^= data
                .binary
                .salt[(16 as libc::c_int + 5 as libc::c_int & 3 as libc::c_int)
                as usize];
        _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
        *ptr.offset(-(2 as libc::c_int as isize)) = L;
        *ptr.offset(-(1 as libc::c_int as isize)) = R;
        if !(ptr
            < &mut *(*(data.ctx.s).as_mut_ptr().offset(3 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0xff as libc::c_int as isize) as *mut uint32_t)
        {
            break;
        }
    }
    loop {
        let mut done: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int + 2 as libc::c_int {
            data.ctx.p[i as usize] ^= data.expanded_key[i as usize];
            data.ctx.p[(i + 1 as libc::c_int) as usize]
                ^= data.expanded_key[(i + 1 as libc::c_int) as usize];
            i += 2 as libc::c_int;
        }
        done = 0 as libc::c_int;
        loop {
            let mut tmp1: uint32_t = 0;
            let mut tmp2: uint32_t = 0;
            let mut tmp3: uint32_t = 0;
            let mut tmp4: uint32_t = 0;
            R = 0 as libc::c_int as uint32_t;
            L = R;
            ptr = (data.ctx.p).as_mut_ptr();
            loop {
                ptr = ptr.offset(2 as libc::c_int as isize);
                _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
                *ptr.offset(-(2 as libc::c_int as isize)) = L;
                *ptr.offset(-(1 as libc::c_int as isize)) = R;
                if !(ptr
                    < &mut *(data.ctx.p)
                        .as_mut_ptr()
                        .offset((16 as libc::c_int + 2 as libc::c_int) as isize)
                        as *mut uint32_t)
                {
                    break;
                }
            }
            ptr = (data.ctx.s[0 as libc::c_int as usize]).as_mut_ptr();
            loop {
                ptr = ptr.offset(2 as libc::c_int as isize);
                _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
                *ptr.offset(-(2 as libc::c_int as isize)) = L;
                *ptr.offset(-(1 as libc::c_int as isize)) = R;
                if !(ptr
                    < &mut *(*(data.ctx.s)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0xff as libc::c_int as isize) as *mut uint32_t)
                {
                    break;
                }
            }
            if done != 0 {
                break;
            }
            done = 1 as libc::c_int;
            tmp1 = data.binary.salt[0 as libc::c_int as usize];
            tmp2 = data.binary.salt[1 as libc::c_int as usize];
            tmp3 = data.binary.salt[2 as libc::c_int as usize];
            tmp4 = data.binary.salt[3 as libc::c_int as usize];
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                data.ctx.p[i as usize] ^= tmp1;
                data.ctx.p[(i + 1 as libc::c_int) as usize] ^= tmp2;
                data.ctx.p[(i + 2 as libc::c_int) as usize] ^= tmp3;
                data.ctx.p[(i + 3 as libc::c_int) as usize] ^= tmp4;
                i += 4 as libc::c_int;
            }
            data.ctx.p[16 as libc::c_int as usize] ^= tmp1;
            data.ctx.p[17 as libc::c_int as usize] ^= tmp2;
        }
        count = count.wrapping_sub(1);
        if !(count != 0) {
            break;
        }
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        L = magic_w[i as usize];
        R = magic_w[(i + 1 as libc::c_int) as usize];
        count = 64 as libc::c_int as uint32_t;
        loop {
            _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
            count = count.wrapping_sub(1);
            if !(count != 0) {
                break;
            }
        }
        data.binary.output[i as usize] = L;
        data.binary.output[(i + 1 as libc::c_int) as usize] = R;
        i += 2 as libc::c_int;
    }
    let fresh9 = dst;
    dst = dst.offset(1);
    *fresh9 = '$' as i32 as uint8_t;
    let fresh10 = dst;
    dst = dst.offset(1);
    *fresh10 = '2' as i32 as uint8_t;
    let fresh11 = dst;
    dst = dst.offset(1);
    *fresh11 = cscheme as uint8_t;
    let fresh12 = dst;
    dst = dst.offset(1);
    *fresh12 = '$' as i32 as uint8_t;
    let fresh13 = dst;
    dst = dst.offset(1);
    *fresh13 = ('0' as i32 + log2rounds / 10 as libc::c_int) as uint8_t;
    let fresh14 = dst;
    dst = dst.offset(1);
    *fresh14 = ('0' as i32 + log2rounds % 10 as libc::c_int) as uint8_t;
    let fresh15 = dst;
    dst = dst.offset(1);
    *fresh15 = '$' as i32 as uint8_t;
    dst = (encode_radix64(
        dst as *mut libc::c_char,
        16 as libc::c_int as size_t,
        psalt.as_mut_ptr(),
    ) as *mut uint8_t)
        .offset(-(1 as libc::c_int as isize));
    bswap32_if_le((data.binary.output).as_mut_ptr(), 6 as libc::c_int as libc::c_uint);
    encode_radix64(
        dst as *mut libc::c_char,
        23 as libc::c_int as size_t,
        (data.binary.output).as_mut_ptr() as *mut uint8_t,
    );
    return cscheme as libc::c_int;
}
pub unsafe extern "C" fn nettle_blowfish_bcrypt_hash(
    mut dst: *mut uint8_t,
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut lenscheme: size_t,
    mut scheme: *const uint8_t,
    mut log2rounds: libc::c_int,
    mut salt: *const uint8_t,
) -> libc::c_int {
    let test_pw: [uint8_t; 10] = *::std::mem::transmute::<
        &[u8; 10],
        &[uint8_t; 10],
    >(b"8b \xD0\xC1\xD2\xCF\xCC\xD8\0");
    let test_scheme: [uint8_t; 30] = *::std::mem::transmute::<
        &[u8; 30],
        &[uint8_t; 30],
    >(b"$2a$00$abcdefghijklmnopqrstuu\0");
    static mut test_hashes: [*const libc::c_char; 2] = [
        b"i1D709vfamulimlGcq0qq3UvuUasvEa\0U\0" as *const u8 as *const libc::c_char,
        b"VUrPmXD6q/nVSSp7pNDhCR9071IfIRe\0U\0" as *const u8 as *const libc::c_char,
    ];
    let mut test_hash: *const libc::c_char = test_hashes[0 as libc::c_int as usize];
    let mut cscheme: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    let mut bufs: [uint8_t; 29] = [0; 29];
    let mut bufo: [uint8_t; 61] = [0; 61];
    *dst = '\0' as i32 as uint8_t;
    cscheme = ibcrypt(
        dst,
        lenkey,
        key,
        lenscheme,
        scheme,
        4 as libc::c_int,
        log2rounds,
        salt,
    );
    memcpy(
        bufs.as_mut_ptr() as *mut libc::c_void,
        test_scheme.as_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[uint8_t; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if cscheme != 0 {
        bufs[2 as libc::c_int as usize] = cscheme as uint8_t;
        test_hash = test_hashes[(bufs[2 as libc::c_int as usize] as libc::c_int
            == 'x' as i32) as libc::c_int as usize];
    }
    *bufo.as_mut_ptr() = 0 as libc::c_int as uint8_t;
    ok = ibcrypt(
        bufo.as_mut_ptr(),
        (::std::mem::size_of::<[uint8_t; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test_pw.as_ptr(),
        ::std::mem::size_of::<[uint8_t; 29]>() as libc::c_ulong,
        bufs.as_mut_ptr(),
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as *const uint8_t,
    );
    ok = (ok != 0
        && memcmp(
            bufo.as_mut_ptr() as *const libc::c_void,
            bufs.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[uint8_t; 29]>() as libc::c_ulong,
        ) == 0
        && memcmp(
            bufo
                .as_mut_ptr()
                .offset(
                    (7 as libc::c_int
                        + (16 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int)
                            / 6 as libc::c_int) as isize,
                ) as *const libc::c_void,
            test_hash as *const libc::c_void,
            (::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0) as libc::c_int;
    let k: [uint8_t; 12] = *::std::mem::transmute::<
        &[u8; 12],
        &[uint8_t; 12],
    >(b"\xFF\xA334\xFF\xFF\xFF\xA3345\0");
    let mut ae: bf_key = [0; 18];
    let mut ai: bf_key = [0; 18];
    let mut ye: bf_key = [0; 18];
    let mut yi: bf_key = [0; 18];
    set_xkey(
        (::std::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        k.as_ptr(),
        ae.as_mut_ptr(),
        ai.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        0x10000 as libc::c_int as uint32_t,
    );
    set_xkey(
        (::std::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        k.as_ptr(),
        ye.as_mut_ptr(),
        yi.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as uint32_t,
    );
    ai[0 as libc::c_int as usize] ^= 0x10000 as libc::c_int as libc::c_uint;
    ok = (ok != 0 && ai[0 as libc::c_int as usize] == 0xdb9c59bc as libc::c_uint
        && ye[17 as libc::c_int as usize] == 0x33343500 as libc::c_int as libc::c_uint
        && memcmp(
            ae.as_mut_ptr() as *const libc::c_void,
            ye.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<bf_key>() as libc::c_ulong,
        ) == 0
        && memcmp(
            ai.as_mut_ptr() as *const libc::c_void,
            yi.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<bf_key>() as libc::c_ulong,
        ) == 0) as libc::c_int;
    return (ok != 0 && cscheme != 0) as libc::c_int;
}
