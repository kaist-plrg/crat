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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arctwo_ctx {
    pub S: [uint16_t; 64],
}
static mut arctwo_sbox: [uint8_t; 256] = [
    0xd9 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xdd as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xed as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0xe9 as libc::c_int as uint8_t,
    0xfd as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x7e as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x4f as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x6d as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0xbd as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0xeb as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x7b as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0x6b as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0xce as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xdc as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0x1f as libc::c_int as uint8_t,
    0x3b as libc::c_int as uint8_t,
    0xbe as libc::c_int as uint8_t,
    0xe4 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0x3d as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x6f as libc::c_int as uint8_t,
    0xbf as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0x27 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xbc as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0xef as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0xe7 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xd5 as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0xd7 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0xea as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
    0xee as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0xd2 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x74 as libc::c_int as uint8_t,
    0x4b as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0x5e as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xec as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0xe0 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x5b as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x5d as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xe3 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0xe6 as libc::c_int as uint8_t,
    0xcf as libc::c_int as uint8_t,
    0xe1 as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0xe2 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0x33 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x5f as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0xe5 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0xfe as libc::c_int as uint8_t,
    0x7f as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
];
pub unsafe extern "C" fn nettle_arctwo_encrypt(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
            b"arctwo.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void nettle_arctwo_encrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1919: {
        if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
                b"arctwo.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"void nettle_arctwo_encrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut i: libc::c_uint = 0;
        let mut w0: uint16_t = 0;
        let mut w1: uint16_t = 0;
        let mut w2: uint16_t = 0;
        let mut w3: uint16_t = 0;
        w0 = ((*(&*src.offset(0 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(0 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        w1 = ((*(&*src.offset(2 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(2 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        w2 = ((*(&*src.offset(4 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(4 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        w3 = ((*(&*src.offset(6 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(6 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 16 as libc::c_int as libc::c_uint {
            let mut j: libc::c_uint = i.wrapping_mul(4 as libc::c_int as libc::c_uint);
            w0 = (w0 as libc::c_int
                + ((w1 as libc::c_int & !(w3 as libc::c_int))
                    + (w2 as libc::c_int & w3 as libc::c_int)
                    + (*ctx).S[j as usize] as libc::c_int)) as uint16_t;
            w0 = ((w0 as libc::c_int) << 1 as libc::c_int as uint16_t as libc::c_int
                | w0 as libc::c_int
                    >> 16 as libc::c_int - 1 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w1 = (w1 as libc::c_int
                + ((w2 as libc::c_int & !(w0 as libc::c_int))
                    + (w3 as libc::c_int & w0 as libc::c_int)
                    + (*ctx).S[j.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int)) as uint16_t;
            w1 = ((w1 as libc::c_int) << 2 as libc::c_int as uint16_t as libc::c_int
                | w1 as libc::c_int
                    >> 16 as libc::c_int - 2 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w2 = (w2 as libc::c_int
                + ((w3 as libc::c_int & !(w1 as libc::c_int))
                    + (w0 as libc::c_int & w1 as libc::c_int)
                    + (*ctx).S[j.wrapping_add(2 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int)) as uint16_t;
            w2 = ((w2 as libc::c_int) << 3 as libc::c_int as uint16_t as libc::c_int
                | w2 as libc::c_int
                    >> 16 as libc::c_int - 3 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w3 = (w3 as libc::c_int
                + ((w0 as libc::c_int & !(w2 as libc::c_int))
                    + (w1 as libc::c_int & w2 as libc::c_int)
                    + (*ctx).S[j.wrapping_add(3 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int)) as uint16_t;
            w3 = ((w3 as libc::c_int) << 5 as libc::c_int as uint16_t as libc::c_int
                | w3 as libc::c_int
                    >> 16 as libc::c_int - 5 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            if i == 4 as libc::c_int as libc::c_uint
                || i == 10 as libc::c_int as libc::c_uint
            {
                w0 = (w0 as libc::c_int
                    + (*ctx).S[(w3 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
                w1 = (w1 as libc::c_int
                    + (*ctx).S[(w0 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
                w2 = (w2 as libc::c_int
                    + (*ctx).S[(w1 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
                w3 = (w3 as libc::c_int
                    + (*ctx).S[(w2 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        *(&mut *dst.offset(0 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w0 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(0 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(2 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w1 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(2 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w1 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(4 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w2 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(4 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w2 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(6 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w3 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(6 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w3 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        length = (length as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(8 as libc::c_int as isize);
        src = src.offset(8 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn nettle_arctwo_decrypt(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
            b"arctwo.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void nettle_arctwo_decrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2655: {
        if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
                b"arctwo.c\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"void nettle_arctwo_decrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut i: libc::c_uint = 0;
        let mut w0: uint16_t = 0;
        let mut w1: uint16_t = 0;
        let mut w2: uint16_t = 0;
        let mut w3: uint16_t = 0;
        w0 = ((*(&*src.offset(0 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(0 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        w1 = ((*(&*src.offset(2 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(2 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        w2 = ((*(&*src.offset(4 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(4 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        w3 = ((*(&*src.offset(6 as libc::c_int as isize) as *const uint8_t)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *(&*src.offset(6 as libc::c_int as isize) as *const uint8_t)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        i = 16 as libc::c_int as libc::c_uint;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_uint) {
                break;
            }
            let mut j: libc::c_uint = i.wrapping_mul(4 as libc::c_int as libc::c_uint);
            w3 = (w3 as libc::c_int >> 5 as libc::c_int as uint16_t as libc::c_int
                | (w3 as libc::c_int)
                    << 16 as libc::c_int - 5 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w3 = (w3 as libc::c_int
                - ((w0 as libc::c_int & !(w2 as libc::c_int))
                    + (w1 as libc::c_int & w2 as libc::c_int)
                    + (*ctx).S[j.wrapping_add(3 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int)) as uint16_t;
            w2 = (w2 as libc::c_int >> 3 as libc::c_int as uint16_t as libc::c_int
                | (w2 as libc::c_int)
                    << 16 as libc::c_int - 3 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w2 = (w2 as libc::c_int
                - ((w3 as libc::c_int & !(w1 as libc::c_int))
                    + (w0 as libc::c_int & w1 as libc::c_int)
                    + (*ctx).S[j.wrapping_add(2 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int)) as uint16_t;
            w1 = (w1 as libc::c_int >> 2 as libc::c_int as uint16_t as libc::c_int
                | (w1 as libc::c_int)
                    << 16 as libc::c_int - 2 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w1 = (w1 as libc::c_int
                - ((w2 as libc::c_int & !(w0 as libc::c_int))
                    + (w3 as libc::c_int & w0 as libc::c_int)
                    + (*ctx).S[j.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int)) as uint16_t;
            w0 = (w0 as libc::c_int >> 1 as libc::c_int as uint16_t as libc::c_int
                | (w0 as libc::c_int)
                    << 16 as libc::c_int - 1 as libc::c_int as uint16_t as libc::c_int)
                as uint16_t;
            w0 = (w0 as libc::c_int
                - ((w1 as libc::c_int & !(w3 as libc::c_int))
                    + (w2 as libc::c_int & w3 as libc::c_int)
                    + (*ctx).S[j as usize] as libc::c_int)) as uint16_t;
            if i == 5 as libc::c_int as libc::c_uint
                || i == 11 as libc::c_int as libc::c_uint
            {
                w3 = (w3 as libc::c_int
                    - (*ctx).S[(w2 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
                w2 = (w2 as libc::c_int
                    - (*ctx).S[(w1 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
                w1 = (w1 as libc::c_int
                    - (*ctx).S[(w0 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
                w0 = (w0 as libc::c_int
                    - (*ctx).S[(w3 as libc::c_int & 63 as libc::c_int) as usize]
                        as libc::c_int) as uint16_t;
            }
        }
        *(&mut *dst.offset(0 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w0 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(0 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(2 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w1 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(2 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w1 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(4 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w2 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(4 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w2 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(6 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                1 as libc::c_int as isize,
            ) = (w3 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(&mut *dst.offset(6 as libc::c_int as isize) as *mut uint8_t)
            .offset(
                0 as libc::c_int as isize,
            ) = (w3 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        length = (length as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(8 as libc::c_int as isize);
        src = src.offset(8 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn nettle_arctwo_set_key_ekb(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
    mut ekb: libc::c_uint,
) {
    let mut i: size_t = 0;
    let mut S: [uint8_t; 128] = [0; 128];
    let mut x: uint8_t = 0;
    if length >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length >= ARCTWO_MIN_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"arctwo.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1104: {
        if length >= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length >= ARCTWO_MIN_KEY_SIZE\0" as *const u8 as *const libc::c_char,
                b"arctwo.c\0" as *const u8 as *const libc::c_char,
                194 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 128 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= ARCTWO_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"arctwo.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1065: {
        if length <= 128 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= ARCTWO_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
                b"arctwo.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ekb <= 1024 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"ekb <= 1024\0" as *const u8 as *const libc::c_char,
            b"arctwo.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1022: {
        if ekb <= 1024 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"ekb <= 1024\0" as *const u8 as *const libc::c_char,
                b"arctwo.c\0" as *const u8 as *const libc::c_char,
                196 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as size_t;
    while i < length {
        S[i as usize] = *key.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = length;
    while i < 128 as libc::c_int as libc::c_ulong {
        S[i
            as usize] = arctwo_sbox[(S[i.wrapping_sub(length) as usize] as libc::c_int
            + S[i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                as libc::c_int & 255 as libc::c_int) as usize];
        i = i.wrapping_add(1);
        i;
    }
    S[0 as libc::c_int as usize] = arctwo_sbox[S[0 as libc::c_int as usize] as usize];
    if ekb > 0 as libc::c_int as libc::c_uint
        && ekb < 1024 as libc::c_int as libc::c_uint
    {
        let mut len: libc::c_int = (ekb.wrapping_add(7 as libc::c_int as libc::c_uint)
            >> 3 as libc::c_int) as libc::c_int;
        i = (128 as libc::c_int - len) as size_t;
        x = arctwo_sbox[(S[i as usize] as libc::c_int
            & 255 as libc::c_int
                >> (7 as libc::c_int as libc::c_uint & ekb.wrapping_neg())) as usize];
        S[i as usize] = x;
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            x = arctwo_sbox[(x as libc::c_int
                ^ S[i.wrapping_add(len as libc::c_ulong) as usize] as libc::c_int)
                as usize];
            S[i as usize] = x;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < 64 as libc::c_int as libc::c_ulong {
        (*ctx)
            .S[i
            as usize] = ((*S
            .as_mut_ptr()
            .offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *S
                .as_mut_ptr()
                .offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
                .offset(0 as libc::c_int as isize) as uint32_t) as uint16_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_arctwo_set_key(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(
        ctx,
        length,
        key,
        (8 as libc::c_int as libc::c_ulong).wrapping_mul(length) as libc::c_uint,
    );
}
pub unsafe extern "C" fn nettle_arctwo_set_key_gutmann(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(ctx, length, key, 0 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn nettle_arctwo40_set_key(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(
        ctx,
        5 as libc::c_int as size_t,
        key,
        40 as libc::c_int as libc::c_uint,
    );
}
pub unsafe extern "C" fn nettle_arctwo64_set_key(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(
        ctx,
        8 as libc::c_int as size_t,
        key,
        64 as libc::c_int as libc::c_uint,
    );
}
pub unsafe extern "C" fn nettle_arctwo128_set_key(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(
        ctx,
        16 as libc::c_int as size_t,
        key,
        128 as libc::c_int as libc::c_uint,
    );
}
pub unsafe extern "C" fn nettle_arctwo128_set_key_gutmann(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(
        ctx,
        16 as libc::c_int as size_t,
        key,
        1024 as libc::c_int as libc::c_uint,
    );
}
