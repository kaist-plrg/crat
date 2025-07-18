use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_write_le32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
    static _nettle_gost28147_param_test_3411: gost28147_param;
    static _nettle_gost28147_param_CryptoPro_3411: gost28147_param;
    fn _nettle_gost28147_encrypt_block(
        key: *const uint32_t,
        sbox: *const [uint32_t; 256],
        in_0: *const uint32_t,
        out: *mut uint32_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gosthash94_ctx {
    pub hash: [uint32_t; 8],
    pub sum: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gost28147_param {
    pub sbox: [[uint32_t; 256]; 4],
}
pub unsafe extern "C" fn nettle_gosthash94_init(mut ctx: *mut gosthash94_ctx) {
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gosthash94_ctx>() as libc::c_ulong,
    );
}
unsafe extern "C" fn gost_block_compress(
    mut ctx: *mut gosthash94_ctx,
    mut block: *const uint32_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut i: libc::c_uint = 0;
    let mut key: [uint32_t; 8] = [0; 8];
    let mut u: [uint32_t; 8] = [0; 8];
    let mut v: [uint32_t; 8] = [0; 8];
    let mut w: [uint32_t; 8] = [0; 8];
    let mut s: [uint32_t; 8] = [0; 8];
    memcpy(
        u.as_mut_ptr() as *mut libc::c_void,
        ((*ctx).hash).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    memcpy(
        v.as_mut_ptr() as *mut libc::c_void,
        block as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    w[0 as libc::c_int
        as usize] = u[0 as libc::c_int as usize] ^ v[0 as libc::c_int as usize];
    w[1 as libc::c_int
        as usize] = u[1 as libc::c_int as usize] ^ v[1 as libc::c_int as usize];
    w[2 as libc::c_int
        as usize] = u[2 as libc::c_int as usize] ^ v[2 as libc::c_int as usize];
    w[3 as libc::c_int
        as usize] = u[3 as libc::c_int as usize] ^ v[3 as libc::c_int as usize];
    w[4 as libc::c_int
        as usize] = u[4 as libc::c_int as usize] ^ v[4 as libc::c_int as usize];
    w[5 as libc::c_int
        as usize] = u[5 as libc::c_int as usize] ^ v[5 as libc::c_int as usize];
    w[6 as libc::c_int
        as usize] = u[6 as libc::c_int as usize] ^ v[6 as libc::c_int as usize];
    w[7 as libc::c_int
        as usize] = u[7 as libc::c_int as usize] ^ v[7 as libc::c_int as usize];
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        key[0 as libc::c_int
            as usize] = w[0 as libc::c_int as usize]
            & 0xff as libc::c_int as libc::c_uint
            | (w[2 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
                << 8 as libc::c_int
            | (w[4 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
                << 16 as libc::c_int
            | (w[6 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
                << 24 as libc::c_int;
        key[1 as libc::c_int
            as usize] = (w[0 as libc::c_int as usize]
            & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int
            | w[2 as libc::c_int as usize] & 0xff00 as libc::c_int as libc::c_uint
            | (w[4 as libc::c_int as usize] & 0xff00 as libc::c_int as libc::c_uint)
                << 8 as libc::c_int
            | (w[6 as libc::c_int as usize] & 0xff00 as libc::c_int as libc::c_uint)
                << 16 as libc::c_int;
        key[2 as libc::c_int
            as usize] = (w[0 as libc::c_int as usize]
            & 0xff0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int
            | (w[2 as libc::c_int as usize] & 0xff0000 as libc::c_int as libc::c_uint)
                >> 8 as libc::c_int
            | w[4 as libc::c_int as usize] & 0xff0000 as libc::c_int as libc::c_uint
            | (w[6 as libc::c_int as usize] & 0xff0000 as libc::c_int as libc::c_uint)
                << 8 as libc::c_int;
        key[3 as libc::c_int
            as usize] = (w[0 as libc::c_int as usize] & 0xff000000 as libc::c_uint)
            >> 24 as libc::c_int
            | (w[2 as libc::c_int as usize] & 0xff000000 as libc::c_uint)
                >> 16 as libc::c_int
            | (w[4 as libc::c_int as usize] & 0xff000000 as libc::c_uint)
                >> 8 as libc::c_int
            | w[6 as libc::c_int as usize] & 0xff000000 as libc::c_uint;
        key[4 as libc::c_int
            as usize] = w[1 as libc::c_int as usize]
            & 0xff as libc::c_int as libc::c_uint
            | (w[3 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
                << 8 as libc::c_int
            | (w[5 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
                << 16 as libc::c_int
            | (w[7 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
                << 24 as libc::c_int;
        key[5 as libc::c_int
            as usize] = (w[1 as libc::c_int as usize]
            & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int
            | w[3 as libc::c_int as usize] & 0xff00 as libc::c_int as libc::c_uint
            | (w[5 as libc::c_int as usize] & 0xff00 as libc::c_int as libc::c_uint)
                << 8 as libc::c_int
            | (w[7 as libc::c_int as usize] & 0xff00 as libc::c_int as libc::c_uint)
                << 16 as libc::c_int;
        key[6 as libc::c_int
            as usize] = (w[1 as libc::c_int as usize]
            & 0xff0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int
            | (w[3 as libc::c_int as usize] & 0xff0000 as libc::c_int as libc::c_uint)
                >> 8 as libc::c_int
            | w[5 as libc::c_int as usize] & 0xff0000 as libc::c_int as libc::c_uint
            | (w[7 as libc::c_int as usize] & 0xff0000 as libc::c_int as libc::c_uint)
                << 8 as libc::c_int;
        key[7 as libc::c_int
            as usize] = (w[1 as libc::c_int as usize] & 0xff000000 as libc::c_uint)
            >> 24 as libc::c_int
            | (w[3 as libc::c_int as usize] & 0xff000000 as libc::c_uint)
                >> 16 as libc::c_int
            | (w[5 as libc::c_int as usize] & 0xff000000 as libc::c_uint)
                >> 8 as libc::c_int
            | w[7 as libc::c_int as usize] & 0xff000000 as libc::c_uint;
        _nettle_gost28147_encrypt_block(
            key.as_mut_ptr(),
            sbox,
            &mut *((*ctx).hash).as_mut_ptr().offset(i as isize),
            &mut *s.as_mut_ptr().offset(i as isize),
        );
        if i == 0 as libc::c_int as libc::c_uint {
            w[0 as libc::c_int
                as usize] = u[2 as libc::c_int as usize] ^ v[4 as libc::c_int as usize];
            w[1 as libc::c_int
                as usize] = u[3 as libc::c_int as usize] ^ v[5 as libc::c_int as usize];
            w[2 as libc::c_int
                as usize] = u[4 as libc::c_int as usize] ^ v[6 as libc::c_int as usize];
            w[3 as libc::c_int
                as usize] = u[5 as libc::c_int as usize] ^ v[7 as libc::c_int as usize];
            v[0 as libc::c_int as usize] ^= v[2 as libc::c_int as usize];
            w[4 as libc::c_int
                as usize] = u[6 as libc::c_int as usize] ^ v[0 as libc::c_int as usize];
            v[1 as libc::c_int as usize] ^= v[3 as libc::c_int as usize];
            w[5 as libc::c_int
                as usize] = u[7 as libc::c_int as usize] ^ v[1 as libc::c_int as usize];
            u[0 as libc::c_int as usize] ^= u[2 as libc::c_int as usize];
            v[2 as libc::c_int as usize] ^= v[4 as libc::c_int as usize];
            w[6 as libc::c_int
                as usize] = u[0 as libc::c_int as usize] ^ v[2 as libc::c_int as usize];
            u[1 as libc::c_int as usize] ^= u[3 as libc::c_int as usize];
            v[3 as libc::c_int as usize] ^= v[5 as libc::c_int as usize];
            w[7 as libc::c_int
                as usize] = u[1 as libc::c_int as usize] ^ v[3 as libc::c_int as usize];
        } else if i & 2 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            if i == 6 as libc::c_int as libc::c_uint {
                break;
            }
            u[2 as libc::c_int as usize]
                ^= u[4 as libc::c_int as usize] ^ 0xff as libc::c_int as libc::c_uint;
            u[3 as libc::c_int as usize]
                ^= u[5 as libc::c_int as usize] ^ 0xff00ffff as libc::c_uint;
            u[4 as libc::c_int as usize] ^= 0xff00ff00 as libc::c_uint;
            u[5 as libc::c_int as usize] ^= 0xff00ff00 as libc::c_uint;
            u[6 as libc::c_int as usize] ^= 0xff00ff as libc::c_int as libc::c_uint;
            u[7 as libc::c_int as usize] ^= 0xff00ff as libc::c_int as libc::c_uint;
            u[0 as libc::c_int as usize] ^= 0xffff00 as libc::c_int as libc::c_uint;
            u[1 as libc::c_int as usize] ^= 0xff0000ff as libc::c_uint;
            w[0 as libc::c_int
                as usize] = u[4 as libc::c_int as usize] ^ v[0 as libc::c_int as usize];
            w[2 as libc::c_int
                as usize] = u[6 as libc::c_int as usize] ^ v[2 as libc::c_int as usize];
            v[4 as libc::c_int as usize] ^= v[6 as libc::c_int as usize];
            w[4 as libc::c_int
                as usize] = u[0 as libc::c_int as usize] ^ v[4 as libc::c_int as usize];
            v[6 as libc::c_int as usize] ^= v[0 as libc::c_int as usize];
            w[6 as libc::c_int
                as usize] = u[2 as libc::c_int as usize] ^ v[6 as libc::c_int as usize];
            w[1 as libc::c_int
                as usize] = u[5 as libc::c_int as usize] ^ v[1 as libc::c_int as usize];
            w[3 as libc::c_int
                as usize] = u[7 as libc::c_int as usize] ^ v[3 as libc::c_int as usize];
            v[5 as libc::c_int as usize] ^= v[7 as libc::c_int as usize];
            w[5 as libc::c_int
                as usize] = u[1 as libc::c_int as usize] ^ v[5 as libc::c_int as usize];
            v[7 as libc::c_int as usize] ^= v[1 as libc::c_int as usize];
            w[7 as libc::c_int
                as usize] = u[3 as libc::c_int as usize] ^ v[7 as libc::c_int as usize];
        } else {
            w[0 as libc::c_int
                as usize] = u[6 as libc::c_int as usize] ^ v[4 as libc::c_int as usize];
            w[1 as libc::c_int
                as usize] = u[7 as libc::c_int as usize] ^ v[5 as libc::c_int as usize];
            w[2 as libc::c_int
                as usize] = u[0 as libc::c_int as usize] ^ v[6 as libc::c_int as usize];
            w[3 as libc::c_int
                as usize] = u[1 as libc::c_int as usize] ^ v[7 as libc::c_int as usize];
            v[0 as libc::c_int as usize] ^= v[2 as libc::c_int as usize];
            w[4 as libc::c_int
                as usize] = u[2 as libc::c_int as usize] ^ v[0 as libc::c_int as usize];
            v[1 as libc::c_int as usize] ^= v[3 as libc::c_int as usize];
            w[5 as libc::c_int
                as usize] = u[3 as libc::c_int as usize] ^ v[1 as libc::c_int as usize];
            u[4 as libc::c_int as usize] ^= u[6 as libc::c_int as usize];
            v[2 as libc::c_int as usize] ^= v[4 as libc::c_int as usize];
            w[6 as libc::c_int
                as usize] = u[4 as libc::c_int as usize] ^ v[2 as libc::c_int as usize];
            u[5 as libc::c_int as usize] ^= u[7 as libc::c_int as usize];
            v[3 as libc::c_int as usize] ^= v[5 as libc::c_int as usize];
            w[7 as libc::c_int
                as usize] = u[5 as libc::c_int as usize] ^ v[3 as libc::c_int as usize];
        }
        i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    u[0 as libc::c_int
        as usize] = *block.offset(0 as libc::c_int as isize)
        ^ s[6 as libc::c_int as usize];
    u[1 as libc::c_int
        as usize] = *block.offset(1 as libc::c_int as isize)
        ^ s[7 as libc::c_int as usize];
    u[2 as libc::c_int
        as usize] = *block.offset(2 as libc::c_int as isize)
        ^ s[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[0 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[1 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize]
        ^ s[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ s[7 as libc::c_int as usize] >> 16 as libc::c_int;
    u[3 as libc::c_int
        as usize] = *block.offset(3 as libc::c_int as isize)
        ^ s[0 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[1 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize]
        ^ s[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] >> 16 as libc::c_int;
    u[4 as libc::c_int
        as usize] = *block.offset(4 as libc::c_int as isize)
        ^ s[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ s[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[1 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ s[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] >> 16 as libc::c_int;
    u[5 as libc::c_int
        as usize] = *block.offset(5 as libc::c_int as isize)
        ^ s[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ s[1 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[2 as libc::c_int as usize]
        ^ s[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ s[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] >> 16 as libc::c_int;
    u[6 as libc::c_int
        as usize] = *block.offset(6 as libc::c_int as isize)
        ^ s[0 as libc::c_int as usize]
        ^ s[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[3 as libc::c_int as usize]
        ^ s[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[6 as libc::c_int as usize]
        ^ s[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] << 16 as libc::c_int;
    u[7 as libc::c_int
        as usize] = *block.offset(7 as libc::c_int as isize)
        ^ s[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ s[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[1 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[4 as libc::c_int as usize]
        ^ s[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ s[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ s[7 as libc::c_int as usize] >> 16 as libc::c_int;
    v[0 as libc::c_int
        as usize] = (*ctx).hash[0 as libc::c_int as usize]
        ^ u[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[0 as libc::c_int as usize] >> 16 as libc::c_int;
    v[1 as libc::c_int
        as usize] = (*ctx).hash[1 as libc::c_int as usize]
        ^ u[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[1 as libc::c_int as usize] >> 16 as libc::c_int;
    v[2 as libc::c_int
        as usize] = (*ctx).hash[2 as libc::c_int as usize]
        ^ u[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[2 as libc::c_int as usize] >> 16 as libc::c_int;
    v[3 as libc::c_int
        as usize] = (*ctx).hash[3 as libc::c_int as usize]
        ^ u[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[3 as libc::c_int as usize] >> 16 as libc::c_int;
    v[4 as libc::c_int
        as usize] = (*ctx).hash[4 as libc::c_int as usize]
        ^ u[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[4 as libc::c_int as usize] >> 16 as libc::c_int;
    v[5 as libc::c_int
        as usize] = (*ctx).hash[5 as libc::c_int as usize]
        ^ u[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[5 as libc::c_int as usize] >> 16 as libc::c_int;
    v[6 as libc::c_int
        as usize] = (*ctx).hash[6 as libc::c_int as usize]
        ^ u[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[6 as libc::c_int as usize] >> 16 as libc::c_int;
    v[7 as libc::c_int
        as usize] = (*ctx).hash[7 as libc::c_int as usize]
        ^ u[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ u[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[1 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ u[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ u[7 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ u[7 as libc::c_int as usize] >> 16 as libc::c_int;
    (*ctx)
        .hash[0 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[5 as libc::c_int as usize]
        ^ v[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint;
    (*ctx)
        .hash[1 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[1 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ v[2 as libc::c_int as usize]
        ^ v[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize]
        ^ v[7 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[7 as libc::c_int as usize] >> 16 as libc::c_int;
    (*ctx)
        .hash[2 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ v[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[3 as libc::c_int as usize]
        ^ v[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[6 as libc::c_int as usize]
        ^ v[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ v[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] >> 16 as libc::c_int;
    (*ctx)
        .hash[3 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[1 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[2 as libc::c_int as usize]
        ^ v[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[4 as libc::c_int as usize]
        ^ v[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] & 0xffff as libc::c_int as libc::c_uint
        ^ v[7 as libc::c_int as usize] >> 16 as libc::c_int;
    (*ctx)
        .hash[4 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[1 as libc::c_int as usize]
        ^ v[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[2 as libc::c_int as usize]
        ^ v[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[3 as libc::c_int as usize]
        ^ v[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[5 as libc::c_int as usize]
        ^ v[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] << 16 as libc::c_int;
    (*ctx)
        .hash[5 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[0 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] & 0xffff0000 as libc::c_uint
        ^ v[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[2 as libc::c_int as usize]
        ^ v[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[3 as libc::c_int as usize]
        ^ v[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[4 as libc::c_int as usize]
        ^ v[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[6 as libc::c_int as usize]
        ^ v[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] & 0xffff0000 as libc::c_uint;
    (*ctx)
        .hash[6 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] ^ v[2 as libc::c_int as usize]
        ^ v[2 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[3 as libc::c_int as usize]
        ^ v[3 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[4 as libc::c_int as usize]
        ^ v[4 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[5 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[5 as libc::c_int as usize]
        ^ v[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[6 as libc::c_int as usize]
        ^ v[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[7 as libc::c_int as usize];
    (*ctx)
        .hash[7 as libc::c_int
        as usize] = v[0 as libc::c_int as usize]
        ^ v[0 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[1 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[2 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[3 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[3 as libc::c_int as usize]
        ^ v[4 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[4 as libc::c_int as usize]
        ^ v[5 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[5 as libc::c_int as usize]
        ^ v[6 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[6 as libc::c_int as usize] >> 16 as libc::c_int
        ^ v[7 as libc::c_int as usize] << 16 as libc::c_int
        ^ v[7 as libc::c_int as usize];
}
unsafe extern "C" fn gost_compute_sum_and_hash(
    mut ctx: *mut gosthash94_ctx,
    mut block: *const uint8_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut block_le: [uint32_t; 8] = [0; 8];
    let mut i: libc::c_uint = 0;
    let mut carry: libc::c_uint = 0;
    carry = 0 as libc::c_int as libc::c_uint;
    i = carry;
    while i < 8 as libc::c_int as libc::c_uint {
        block_le[i
            as usize] = (*block.offset(3 as libc::c_int as isize) as uint32_t)
            << 24 as libc::c_int
            | (*block.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*block.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *block.offset(0 as libc::c_int as isize) as uint32_t;
        (*ctx)
            .sum[i
            as usize] = ((*ctx).sum[i as usize] as libc::c_uint).wrapping_add(carry)
            as uint32_t as uint32_t;
        carry = ((*ctx).sum[i as usize] < carry) as libc::c_int as libc::c_uint;
        (*ctx)
            .sum[i
            as usize] = ((*ctx).sum[i as usize] as libc::c_uint)
            .wrapping_add(block_le[i as usize]) as uint32_t as uint32_t;
        carry = carry
            .wrapping_add(
                ((*ctx).sum[i as usize] < block_le[i as usize]) as libc::c_int
                    as libc::c_uint,
            );
        i = i.wrapping_add(1);
        i;
        block = block.offset(4 as libc::c_int as isize);
    }
    gost_block_compress(ctx, block_le.as_mut_ptr(), sbox);
}
unsafe extern "C" fn gosthash94_update_int(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut msg: *const uint8_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut current_block_18: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 32]>()
            as libc::c_ulong)
            .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
        if length < __md_left as libc::c_ulong {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                msg as *const libc::c_void,
                length,
            );
            (*ctx)
                .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                as libc::c_uint as libc::c_uint;
            current_block_18 = 15976848397966268834;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                msg as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            gost_compute_sum_and_hash(ctx, ((*ctx).block).as_mut_ptr(), sbox);
            (*ctx).count = ((*ctx).count).wrapping_add(1);
            (*ctx).count;
            msg = msg.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_18 = 1394248824506584008;
        }
    } else {
        current_block_18 = 1394248824506584008;
    }
    match current_block_18 {
        1394248824506584008 => {
            while length >= ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong {
                gost_compute_sum_and_hash(ctx, msg, sbox);
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                msg = msg
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            memcpy(
                ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                msg as *const libc::c_void,
                length,
            );
            (*ctx).index = length as libc::c_uint;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn nettle_gosthash94_update(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut msg: *const uint8_t,
) {
    gosthash94_update_int(
        ctx,
        length,
        msg,
        (_nettle_gost28147_param_test_3411.sbox).as_ptr(),
    );
}
pub unsafe extern "C" fn nettle_gosthash94cp_update(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut msg: *const uint8_t,
) {
    gosthash94_update_int(
        ctx,
        length,
        msg,
        (_nettle_gost28147_param_CryptoPro_3411.sbox).as_ptr(),
    );
}
unsafe extern "C" fn gosthash94_write_digest(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut result: *mut uint8_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut msg32: [uint32_t; 8] = [0; 8];
    if length <= 32 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= GOSTHASH94_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"gosthash94.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void gosthash94_write_digest(struct gosthash94_ctx *, size_t, uint8_t *, const uint32_t (*)[256])\0",
            ))
                .as_ptr(),
        );
    }
    'c_4538: {
        if length <= 32 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= GOSTHASH94_DIGEST_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"gosthash94.c\0" as *const u8 as *const libc::c_char,
                349 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void gosthash94_write_digest(struct gosthash94_ctx *, size_t, uint8_t *, const uint32_t (*)[256])\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).index > 0 as libc::c_int as libc::c_uint {
        memset(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int as libc::c_uint).wrapping_sub((*ctx).index)
                as libc::c_ulong,
        );
        gost_compute_sum_and_hash(ctx, ((*ctx).block).as_mut_ptr(), sbox);
    }
    msg32[0 as libc::c_int
        as usize] = ((*ctx).count << 8 as libc::c_int
        | ((*ctx).index << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
    msg32[1 as libc::c_int as usize] = ((*ctx).count >> 24 as libc::c_int) as uint32_t;
    memset(
        msg32.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(6 as libc::c_int as libc::c_ulong),
    );
    gost_block_compress(ctx, msg32.as_mut_ptr(), sbox);
    gost_block_compress(ctx, ((*ctx).sum).as_mut_ptr(), sbox);
    _nettle_write_le32(length, result, ((*ctx).hash).as_mut_ptr());
    nettle_gosthash94_init(ctx);
}
pub unsafe extern "C" fn nettle_gosthash94_digest(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut result: *mut uint8_t,
) {
    gosthash94_write_digest(
        ctx,
        length,
        result,
        (_nettle_gost28147_param_test_3411.sbox).as_ptr(),
    );
}
pub unsafe extern "C" fn nettle_gosthash94cp_digest(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut result: *mut uint8_t,
) {
    gosthash94_write_digest(
        ctx,
        length,
        result,
        (_nettle_gost28147_param_CryptoPro_3411.sbox).as_ptr(),
    );
}
