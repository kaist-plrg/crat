use ::libc;
extern "C" {
    static _nettle_aes_encrypt_table: aes_table;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_table {
    pub sbox: [uint8_t; 256],
    pub table: [[uint32_t; 256]; 4],
}
pub unsafe extern "C" fn _nettle_aes_set_key(
    mut nr: libc::c_uint,
    mut nk: libc::c_uint,
    mut subkeys: *mut uint32_t,
    mut key: *const uint8_t,
) {
    static mut rcon: [uint8_t; 10] = [
        0x1 as libc::c_int as uint8_t,
        0x2 as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
        0x8 as libc::c_int as uint8_t,
        0x10 as libc::c_int as uint8_t,
        0x20 as libc::c_int as uint8_t,
        0x40 as libc::c_int as uint8_t,
        0x80 as libc::c_int as uint8_t,
        0x1b as libc::c_int as uint8_t,
        0x36 as libc::c_int as uint8_t,
    ];
    let mut rp: *const uint8_t = 0 as *const uint8_t;
    let mut lastkey: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut t: uint32_t = 0;
    if nk != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"nk != 0\0" as *const u8 as *const libc::c_char,
            b"aes-set-key-internal.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void _nettle_aes_set_key(unsigned int, unsigned int, uint32_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_828: {
        if nk != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"nk != 0\0" as *const u8 as *const libc::c_char,
                b"aes-set-key-internal.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void _nettle_aes_set_key(unsigned int, unsigned int, uint32_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    lastkey = ((16 as libc::c_int / 4 as libc::c_int) as libc::c_uint)
        .wrapping_mul(nr.wrapping_add(1 as libc::c_int as libc::c_uint));
    i = 0 as libc::c_int as libc::c_uint;
    rp = rcon.as_ptr();
    while i < nk {
        *subkeys
            .offset(
                i as isize,
            ) = (*key
            .offset(i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*key
                .offset(i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize)
                .offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*key
                .offset(i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *key
                .offset(i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize)
                .offset(0 as libc::c_int as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    i = nk;
    while i < lastkey {
        t = *subkeys.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        if i.wrapping_rem(nk) == 0 as libc::c_int as libc::c_uint {
            let fresh0 = rp;
            rp = rp.offset(1);
            t = (_nettle_aes_encrypt_table
                .sbox[((t << 24 as libc::c_int
                | t >> (-(24 as libc::c_int) & 31 as libc::c_int))
                & 0xff as libc::c_int as libc::c_uint) as usize] as uint32_t
                | (_nettle_aes_encrypt_table
                    .sbox[((t << 24 as libc::c_int
                    | t >> (-(24 as libc::c_int) & 31 as libc::c_int))
                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize]
                    as uint32_t) << 8 as libc::c_int
                | (_nettle_aes_encrypt_table
                    .sbox[((t << 24 as libc::c_int
                    | t >> (-(24 as libc::c_int) & 31 as libc::c_int))
                    >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize]
                    as uint32_t) << 16 as libc::c_int
                | (_nettle_aes_encrypt_table
                    .sbox[((t << 24 as libc::c_int
                    | t >> (-(24 as libc::c_int) & 31 as libc::c_int))
                    >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize]
                    as uint32_t) << 24 as libc::c_int) ^ *fresh0 as libc::c_uint;
        } else if nk > 6 as libc::c_int as libc::c_uint
            && i.wrapping_rem(nk) == 4 as libc::c_int as libc::c_uint
        {
            t = _nettle_aes_encrypt_table
                .sbox[(t & 0xff as libc::c_int as libc::c_uint) as usize] as uint32_t
                | (_nettle_aes_encrypt_table
                    .sbox[(t >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as usize] as uint32_t) << 8 as libc::c_int
                | (_nettle_aes_encrypt_table
                    .sbox[(t >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as usize] as uint32_t) << 16 as libc::c_int
                | (_nettle_aes_encrypt_table
                    .sbox[(t >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as usize] as uint32_t) << 24 as libc::c_int;
        }
        *subkeys.offset(i as isize) = *subkeys.offset(i.wrapping_sub(nk) as isize) ^ t;
        i = i.wrapping_add(1);
        i;
    }
}
