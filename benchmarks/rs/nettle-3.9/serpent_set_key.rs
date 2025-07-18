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
pub struct serpent_ctx {
    pub keys: [[uint32_t; 4]; 33],
}
unsafe extern "C" fn serpent_key_pad(
    mut key: *const uint8_t,
    mut key_length: libc::c_uint,
    mut w: *mut uint32_t,
) {
    let mut i: libc::c_uint = 0;
    if key_length <= 32 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"key_length <= SERPENT_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"serpent-set-key.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void serpent_key_pad(const uint8_t *, unsigned int, uint32_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5873: {
        if key_length <= 32 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"key_length <= SERPENT_MAX_KEY_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"serpent-set-key.c\0" as *const u8 as *const libc::c_char,
                303 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void serpent_key_pad(const uint8_t *, unsigned int, uint32_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as libc::c_uint;
    while key_length >= 4 as libc::c_int as libc::c_uint {
        let fresh0 = i;
        i = i.wrapping_add(1);
        *w
            .offset(
                fresh0 as isize,
            ) = (*key.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*key.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*key.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *key.offset(0 as libc::c_int as isize) as uint32_t;
        key_length = key_length.wrapping_sub(4 as libc::c_int as libc::c_uint);
        key = key.offset(4 as libc::c_int as isize);
    }
    if i < 8 as libc::c_int as libc::c_uint {
        let mut pad: uint32_t = 0x1 as libc::c_int as uint32_t;
        while key_length > 0 as libc::c_int as libc::c_uint {
            key_length = key_length.wrapping_sub(1);
            pad = pad << 8 as libc::c_int
                | *key.offset(key_length as isize) as libc::c_uint;
        }
        let fresh1 = i;
        i = i.wrapping_add(1);
        *w.offset(fresh1 as isize) = pad;
        while i < 8 as libc::c_int as libc::c_uint {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *w.offset(fresh2 as isize) = 0 as libc::c_int as uint32_t;
        }
    }
}
pub unsafe extern "C" fn nettle_serpent_set_key(
    mut ctx: *mut serpent_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    let mut w: [uint32_t; 8] = [0; 8];
    let mut keys: *mut [uint32_t; 4] = 0 as *mut [uint32_t; 4];
    let mut k: libc::c_uint = 0;
    serpent_key_pad(key, length as libc::c_uint, w.as_mut_ptr());
    keys = ((*ctx).keys).as_mut_ptr();
    k = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh3 = k;
        k = k.wrapping_add(1);
        let mut _wn: uint32_t = w[0 as libc::c_int as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh3;
        w[0 as libc::c_int
            as usize] = _wn << 11 as libc::c_int
            | _wn >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh4 = k;
        k = k.wrapping_add(1);
        let mut _wn_0: uint32_t = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh4;
        w[(0 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_0 << 11 as libc::c_int
            | _wn_0 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh5 = k;
        k = k.wrapping_add(1);
        let mut _wn_1: uint32_t = w[(0 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh5;
        w[(0 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_1 << 11 as libc::c_int
            | _wn_1 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh6 = k;
        k = k.wrapping_add(1);
        let mut _wn_2: uint32_t = w[(0 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh6;
        w[(0 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_2 << 11 as libc::c_int
            | _wn_2 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02: uint32_t = 0;
        let mut t03: uint32_t = 0;
        let mut t04: uint32_t = 0;
        let mut t05: uint32_t = 0;
        let mut t06: uint32_t = 0;
        let mut t07: uint32_t = 0;
        let mut t08: uint32_t = 0;
        let mut t09: uint32_t = 0;
        let mut t10: uint32_t = 0;
        let mut t11: uint32_t = 0;
        let mut t13: uint32_t = 0;
        let mut t14: uint32_t = 0;
        let mut t15: uint32_t = 0;
        let mut t01: uint32_t = 0;
        t01 = w[0 as libc::c_int as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int) as usize];
        t02 = w[0 as libc::c_int as usize]
            | w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t03 = w[0 as libc::c_int as usize]
            & w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t04 = t01 & t02;
        t05 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] | t03;
        t06 = w[0 as libc::c_int as usize]
            & w[(0 as libc::c_int + 1 as libc::c_int) as usize];
        t07 = w[(0 as libc::c_int + 3 as libc::c_int) as usize] ^ t04;
        t08 = w[(0 as libc::c_int + 2 as libc::c_int) as usize] | t06;
        t09 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] ^ t07;
        t10 = w[(0 as libc::c_int + 3 as libc::c_int) as usize] & t05;
        t11 = t02 ^ t10;
        (*keys)[3 as libc::c_int as usize] = t08 ^ t09;
        t13 = w[(0 as libc::c_int + 3 as libc::c_int) as usize]
            | (*keys)[3 as libc::c_int as usize];
        t14 = w[0 as libc::c_int as usize] | t07;
        t15 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] & t13;
        (*keys)[2 as libc::c_int as usize] = t08 ^ t11;
        (*keys)[0 as libc::c_int as usize] = t14 ^ t15;
        (*keys)[1 as libc::c_int as usize] = t05 ^ t04;
        keys = keys.offset(1);
        keys;
        if k == 132 as libc::c_int as libc::c_uint {
            break;
        }
        let fresh7 = k;
        k = k.wrapping_add(1);
        let mut _wn_3: uint32_t = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh7;
        w[4 as libc::c_int
            as usize] = _wn_3 << 11 as libc::c_int
            | _wn_3 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh8 = k;
        k = k.wrapping_add(1);
        let mut _wn_4: uint32_t = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh8;
        w[(4 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_4 << 11 as libc::c_int
            | _wn_4 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh9 = k;
        k = k.wrapping_add(1);
        let mut _wn_5: uint32_t = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh9;
        w[(4 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_5 << 11 as libc::c_int
            | _wn_5 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh10 = k;
        k = k.wrapping_add(1);
        let mut _wn_6: uint32_t = w[(4 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh10;
        w[(4 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_6 << 11 as libc::c_int
            | _wn_6 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_0: uint32_t = 0;
        let mut t03_0: uint32_t = 0;
        let mut t05_0: uint32_t = 0;
        let mut t06_0: uint32_t = 0;
        let mut t07_0: uint32_t = 0;
        let mut t08_0: uint32_t = 0;
        let mut t09_0: uint32_t = 0;
        let mut t10_0: uint32_t = 0;
        let mut t12: uint32_t = 0;
        let mut t13_0: uint32_t = 0;
        let mut t14_0: uint32_t = 0;
        let mut t01_0: uint32_t = 0;
        t01_0 = w[4 as libc::c_int as usize]
            | w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t02_0 = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int) as usize];
        t03_0 = w[(4 as libc::c_int + 3 as libc::c_int) as usize] ^ t01_0;
        (*keys)[0 as libc::c_int as usize] = t02_0 ^ t03_0;
        t05_0 = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            ^ (*keys)[0 as libc::c_int as usize];
        t06_0 = w[(4 as libc::c_int + 1 as libc::c_int) as usize] ^ t05_0;
        t07_0 = w[(4 as libc::c_int + 1 as libc::c_int) as usize] | t05_0;
        t08_0 = t01_0 & t06_0;
        t09_0 = t03_0 ^ t07_0;
        t10_0 = t02_0 | t09_0;
        (*keys)[1 as libc::c_int as usize] = t10_0 ^ t08_0;
        t12 = w[4 as libc::c_int as usize]
            | w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t13_0 = t09_0 ^ (*keys)[1 as libc::c_int as usize];
        t14_0 = w[(4 as libc::c_int + 1 as libc::c_int) as usize] ^ t13_0;
        (*keys)[3 as libc::c_int as usize] = !t09_0;
        (*keys)[2 as libc::c_int as usize] = t12 ^ t14_0;
        keys = keys.offset(1);
        keys;
        let fresh11 = k;
        k = k.wrapping_add(1);
        let mut _wn_7: uint32_t = w[0 as libc::c_int as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh11;
        w[0 as libc::c_int
            as usize] = _wn_7 << 11 as libc::c_int
            | _wn_7 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh12 = k;
        k = k.wrapping_add(1);
        let mut _wn_8: uint32_t = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh12;
        w[(0 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_8 << 11 as libc::c_int
            | _wn_8 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh13 = k;
        k = k.wrapping_add(1);
        let mut _wn_9: uint32_t = w[(0 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh13;
        w[(0 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_9 << 11 as libc::c_int
            | _wn_9 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh14 = k;
        k = k.wrapping_add(1);
        let mut _wn_10: uint32_t = w[(0 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh14;
        w[(0 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_10 << 11 as libc::c_int
            | _wn_10 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_1: uint32_t = 0;
        let mut t03_1: uint32_t = 0;
        let mut t04_0: uint32_t = 0;
        let mut t05_1: uint32_t = 0;
        let mut t06_1: uint32_t = 0;
        let mut t07_1: uint32_t = 0;
        let mut t08_1: uint32_t = 0;
        let mut t10_1: uint32_t = 0;
        let mut t11_0: uint32_t = 0;
        let mut t12_0: uint32_t = 0;
        let mut t13_1: uint32_t = 0;
        let mut t16: uint32_t = 0;
        let mut t17: uint32_t = 0;
        let mut t01_1: uint32_t = 0;
        t01_1 = w[0 as libc::c_int as usize]
            | w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t02_1 = w[(0 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t03_1 = !w[(0 as libc::c_int + 1 as libc::c_int) as usize];
        t04_0 = w[0 as libc::c_int as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int) as usize];
        t05_1 = w[0 as libc::c_int as usize] | t03_1;
        t06_1 = w[(0 as libc::c_int + 3 as libc::c_int) as usize] & t04_0;
        t07_1 = t01_1 & t02_1;
        t08_1 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] | t06_1;
        (*keys)[2 as libc::c_int as usize] = t02_1 ^ t05_1;
        t10_1 = t07_1 ^ t08_1;
        t11_0 = t01_1 ^ t10_1;
        t12_0 = (*keys)[2 as libc::c_int as usize] ^ t11_0;
        t13_1 = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            & w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        (*keys)[3 as libc::c_int as usize] = !t10_1;
        (*keys)[1 as libc::c_int as usize] = t13_1 ^ t12_0;
        t16 = t10_1 | (*keys)[1 as libc::c_int as usize];
        t17 = t05_1 & t16;
        (*keys)[0 as libc::c_int
            as usize] = w[(0 as libc::c_int + 2 as libc::c_int) as usize] ^ t17;
        keys = keys.offset(1);
        keys;
        let fresh15 = k;
        k = k.wrapping_add(1);
        let mut _wn_11: uint32_t = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh15;
        w[4 as libc::c_int
            as usize] = _wn_11 << 11 as libc::c_int
            | _wn_11 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh16 = k;
        k = k.wrapping_add(1);
        let mut _wn_12: uint32_t = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh16;
        w[(4 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_12 << 11 as libc::c_int
            | _wn_12 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh17 = k;
        k = k.wrapping_add(1);
        let mut _wn_13: uint32_t = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh17;
        w[(4 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_13 << 11 as libc::c_int
            | _wn_13 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh18 = k;
        k = k.wrapping_add(1);
        let mut _wn_14: uint32_t = w[(4 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh18;
        w[(4 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_14 << 11 as libc::c_int
            | _wn_14 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_2: uint32_t = 0;
        let mut t03_2: uint32_t = 0;
        let mut t05_2: uint32_t = 0;
        let mut t06_2: uint32_t = 0;
        let mut t07_2: uint32_t = 0;
        let mut t08_2: uint32_t = 0;
        let mut t09_1: uint32_t = 0;
        let mut t11_1: uint32_t = 0;
        let mut t12_1: uint32_t = 0;
        let mut t13_2: uint32_t = 0;
        let mut t14_1: uint32_t = 0;
        let mut t15_0: uint32_t = 0;
        let mut t17_0: uint32_t = 0;
        let mut t01_2: uint32_t = 0;
        t01_2 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t02_2 = w[4 as libc::c_int as usize]
            | w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t03_2 = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int) as usize];
        (*keys)[3 as libc::c_int as usize] = t02_2 ^ t01_2;
        t05_2 = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            | (*keys)[3 as libc::c_int as usize];
        t06_2 = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t07_2 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            | w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t08_2 = w[(4 as libc::c_int + 3 as libc::c_int) as usize] & t05_2;
        t09_1 = t03_2 & t07_2;
        (*keys)[2 as libc::c_int as usize] = t09_1 ^ t08_2;
        t11_1 = t09_1 & (*keys)[2 as libc::c_int as usize];
        t12_1 = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t13_2 = t07_2 ^ t11_1;
        t14_1 = w[(4 as libc::c_int + 1 as libc::c_int) as usize] & t06_2;
        t15_0 = t06_2 ^ t13_2;
        (*keys)[0 as libc::c_int as usize] = !t15_0;
        t17_0 = (*keys)[0 as libc::c_int as usize] ^ t14_1;
        (*keys)[1 as libc::c_int as usize] = t12_1 ^ t17_0;
        keys = keys.offset(1);
        keys;
        let fresh19 = k;
        k = k.wrapping_add(1);
        let mut _wn_15: uint32_t = w[0 as libc::c_int as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh19;
        w[0 as libc::c_int
            as usize] = _wn_15 << 11 as libc::c_int
            | _wn_15 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh20 = k;
        k = k.wrapping_add(1);
        let mut _wn_16: uint32_t = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh20;
        w[(0 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_16 << 11 as libc::c_int
            | _wn_16 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh21 = k;
        k = k.wrapping_add(1);
        let mut _wn_17: uint32_t = w[(0 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh21;
        w[(0 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_17 << 11 as libc::c_int
            | _wn_17 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh22 = k;
        k = k.wrapping_add(1);
        let mut _wn_18: uint32_t = w[(0 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh22;
        w[(0 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_18 << 11 as libc::c_int
            | _wn_18 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_3: uint32_t = 0;
        let mut t03_3: uint32_t = 0;
        let mut t04_1: uint32_t = 0;
        let mut t05_3: uint32_t = 0;
        let mut t06_3: uint32_t = 0;
        let mut t08_3: uint32_t = 0;
        let mut t09_2: uint32_t = 0;
        let mut t10_2: uint32_t = 0;
        let mut t11_2: uint32_t = 0;
        let mut t13_3: uint32_t = 0;
        let mut t14_2: uint32_t = 0;
        let mut t15_1: uint32_t = 0;
        let mut t16_0: uint32_t = 0;
        let mut t17_1: uint32_t = 0;
        let mut t01_3: uint32_t = 0;
        t01_3 = w[0 as libc::c_int as usize]
            & w[(0 as libc::c_int + 2 as libc::c_int) as usize];
        t02_3 = !w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t03_3 = w[0 as libc::c_int as usize] & t02_3;
        t04_1 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] | t01_3;
        t05_3 = w[0 as libc::c_int as usize]
            & w[(0 as libc::c_int + 1 as libc::c_int) as usize];
        t06_3 = w[(0 as libc::c_int + 2 as libc::c_int) as usize] ^ t04_1;
        (*keys)[3 as libc::c_int as usize] = t03_3 ^ t06_3;
        t08_3 = w[(0 as libc::c_int + 2 as libc::c_int) as usize]
            | (*keys)[3 as libc::c_int as usize];
        t09_2 = w[(0 as libc::c_int + 3 as libc::c_int) as usize] | t05_3;
        t10_2 = w[0 as libc::c_int as usize] ^ t08_3;
        t11_2 = t04_1 & (*keys)[3 as libc::c_int as usize];
        (*keys)[1 as libc::c_int as usize] = t09_2 ^ t10_2;
        t13_3 = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            ^ (*keys)[1 as libc::c_int as usize];
        t14_2 = t01_3 ^ (*keys)[1 as libc::c_int as usize];
        t15_1 = w[(0 as libc::c_int + 2 as libc::c_int) as usize] ^ t05_3;
        t16_0 = t11_2 | t13_3;
        t17_1 = t02_3 | t14_2;
        (*keys)[0 as libc::c_int as usize] = t15_1 ^ t17_1;
        (*keys)[2 as libc::c_int as usize] = w[0 as libc::c_int as usize] ^ t16_0;
        keys = keys.offset(1);
        keys;
        let fresh23 = k;
        k = k.wrapping_add(1);
        let mut _wn_19: uint32_t = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh23;
        w[4 as libc::c_int
            as usize] = _wn_19 << 11 as libc::c_int
            | _wn_19 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh24 = k;
        k = k.wrapping_add(1);
        let mut _wn_20: uint32_t = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh24;
        w[(4 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_20 << 11 as libc::c_int
            | _wn_20 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh25 = k;
        k = k.wrapping_add(1);
        let mut _wn_21: uint32_t = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh25;
        w[(4 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_21 << 11 as libc::c_int
            | _wn_21 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh26 = k;
        k = k.wrapping_add(1);
        let mut _wn_22: uint32_t = w[(4 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh26;
        w[(4 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_22 << 11 as libc::c_int
            | _wn_22 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_4: uint32_t = 0;
        let mut t03_4: uint32_t = 0;
        let mut t04_2: uint32_t = 0;
        let mut t05_4: uint32_t = 0;
        let mut t07_3: uint32_t = 0;
        let mut t08_4: uint32_t = 0;
        let mut t09_3: uint32_t = 0;
        let mut t10_3: uint32_t = 0;
        let mut t11_3: uint32_t = 0;
        let mut t12_2: uint32_t = 0;
        let mut t13_4: uint32_t = 0;
        let mut t15_2: uint32_t = 0;
        let mut t17_2: uint32_t = 0;
        let mut t18: uint32_t = 0;
        let mut t01_4: uint32_t = 0;
        t01_4 = w[4 as libc::c_int as usize]
            & w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t02_4 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t03_4 = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t04_2 = t01_4 ^ t02_4;
        t05_4 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            | w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        (*keys)[1 as libc::c_int as usize] = !t04_2;
        t07_3 = t03_4 & t05_4;
        t08_4 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            & (*keys)[1 as libc::c_int as usize];
        t09_3 = w[4 as libc::c_int as usize]
            | w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t10_3 = t07_3 ^ t08_4;
        t11_3 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            | w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t12_2 = w[(4 as libc::c_int + 2 as libc::c_int) as usize] ^ t11_3;
        t13_4 = t09_3 ^ t10_3;
        (*keys)[2 as libc::c_int as usize] = !t13_4;
        t15_2 = (*keys)[1 as libc::c_int as usize] & t03_4;
        (*keys)[3 as libc::c_int as usize] = t12_2 ^ t07_3;
        t17_2 = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int) as usize];
        t18 = (*keys)[2 as libc::c_int as usize] ^ t15_2;
        (*keys)[0 as libc::c_int as usize] = t17_2 ^ t18;
        keys = keys.offset(1);
        keys;
        let fresh27 = k;
        k = k.wrapping_add(1);
        let mut _wn_23: uint32_t = w[0 as libc::c_int as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh27;
        w[0 as libc::c_int
            as usize] = _wn_23 << 11 as libc::c_int
            | _wn_23 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh28 = k;
        k = k.wrapping_add(1);
        let mut _wn_24: uint32_t = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh28;
        w[(0 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_24 << 11 as libc::c_int
            | _wn_24 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh29 = k;
        k = k.wrapping_add(1);
        let mut _wn_25: uint32_t = w[(0 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh29;
        w[(0 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_25 << 11 as libc::c_int
            | _wn_25 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh30 = k;
        k = k.wrapping_add(1);
        let mut _wn_26: uint32_t = w[(0 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh30;
        w[(0 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_26 << 11 as libc::c_int
            | _wn_26 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_5: uint32_t = 0;
        let mut t03_5: uint32_t = 0;
        let mut t04_3: uint32_t = 0;
        let mut t05_5: uint32_t = 0;
        let mut t07_4: uint32_t = 0;
        let mut t08_5: uint32_t = 0;
        let mut t09_4: uint32_t = 0;
        let mut t10_4: uint32_t = 0;
        let mut t11_4: uint32_t = 0;
        let mut t12_3: uint32_t = 0;
        let mut t13_5: uint32_t = 0;
        let mut t14_3: uint32_t = 0;
        let mut t01_5: uint32_t = 0;
        t01_5 = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t02_5 = w[(0 as libc::c_int + 1 as libc::c_int) as usize]
            | w[(0 as libc::c_int + 3 as libc::c_int) as usize];
        t03_5 = w[0 as libc::c_int as usize] & t01_5;
        t04_3 = w[(0 as libc::c_int + 2 as libc::c_int) as usize] ^ t02_5;
        t05_5 = t03_5 ^ t04_3;
        (*keys)[0 as libc::c_int as usize] = !t05_5;
        t07_4 = w[0 as libc::c_int as usize] ^ t01_5;
        t08_5 = w[(0 as libc::c_int + 3 as libc::c_int) as usize]
            | (*keys)[0 as libc::c_int as usize];
        t09_4 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] | t05_5;
        t10_4 = w[(0 as libc::c_int + 3 as libc::c_int) as usize] ^ t08_5;
        t11_4 = w[(0 as libc::c_int + 1 as libc::c_int) as usize] | t07_4;
        t12_3 = t03_5 | (*keys)[0 as libc::c_int as usize];
        t13_5 = t07_4 | t10_4;
        t14_3 = t01_5 ^ t11_4;
        (*keys)[2 as libc::c_int as usize] = t09_4 ^ t13_5;
        (*keys)[1 as libc::c_int as usize] = t07_4 ^ t08_5;
        (*keys)[3 as libc::c_int as usize] = t12_3 ^ t14_3;
        keys = keys.offset(1);
        keys;
        let fresh31 = k;
        k = k.wrapping_add(1);
        let mut _wn_27: uint32_t = w[4 as libc::c_int as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 5 as libc::c_int & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 7 as libc::c_int & 7 as libc::c_int) as usize]
            ^ 0x9e3779b9 as libc::c_uint ^ fresh31;
        w[4 as libc::c_int
            as usize] = _wn_27 << 11 as libc::c_int
            | _wn_27 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh32 = k;
        k = k.wrapping_add(1);
        let mut _wn_28: uint32_t = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 1 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh32;
        w[(4 as libc::c_int + 1 as libc::c_int)
            as usize] = _wn_28 << 11 as libc::c_int
            | _wn_28 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh33 = k;
        k = k.wrapping_add(1);
        let mut _wn_29: uint32_t = w[(4 as libc::c_int + 2 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 2 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh33;
        w[(4 as libc::c_int + 2 as libc::c_int)
            as usize] = _wn_29 << 11 as libc::c_int
            | _wn_29 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let fresh34 = k;
        k = k.wrapping_add(1);
        let mut _wn_30: uint32_t = w[(4 as libc::c_int + 3 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int
                & 7 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                & 7 as libc::c_int) as usize] ^ 0x9e3779b9 as libc::c_uint ^ fresh34;
        w[(4 as libc::c_int + 3 as libc::c_int)
            as usize] = _wn_30 << 11 as libc::c_int
            | _wn_30 >> (-(11 as libc::c_int) & 31 as libc::c_int);
        let mut t02_6: uint32_t = 0;
        let mut t03_6: uint32_t = 0;
        let mut t04_4: uint32_t = 0;
        let mut t05_6: uint32_t = 0;
        let mut t06_4: uint32_t = 0;
        let mut t08_6: uint32_t = 0;
        let mut t09_5: uint32_t = 0;
        let mut t10_5: uint32_t = 0;
        let mut t11_5: uint32_t = 0;
        let mut t12_4: uint32_t = 0;
        let mut t13_6: uint32_t = 0;
        let mut t14_4: uint32_t = 0;
        let mut t15_3: uint32_t = 0;
        let mut t16_1: uint32_t = 0;
        let mut t01_6: uint32_t = 0;
        t01_6 = w[4 as libc::c_int as usize]
            | w[(4 as libc::c_int + 1 as libc::c_int) as usize];
        t02_6 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            | w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t03_6 = w[4 as libc::c_int as usize] ^ t02_6;
        t04_4 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            ^ w[(4 as libc::c_int + 3 as libc::c_int) as usize];
        t05_6 = w[(4 as libc::c_int + 3 as libc::c_int) as usize] | t03_6;
        t06_4 = w[(4 as libc::c_int + 3 as libc::c_int) as usize] & t01_6;
        (*keys)[3 as libc::c_int as usize] = t03_6 ^ t06_4;
        t08_6 = (*keys)[3 as libc::c_int as usize] & t04_4;
        t09_5 = t04_4 & t05_6;
        t10_5 = w[(4 as libc::c_int + 2 as libc::c_int) as usize] ^ t06_4;
        t11_5 = w[(4 as libc::c_int + 1 as libc::c_int) as usize]
            & w[(4 as libc::c_int + 2 as libc::c_int) as usize];
        t12_4 = t04_4 ^ t08_6;
        t13_6 = t11_5 | t03_6;
        t14_4 = t10_5 ^ t09_5;
        t15_3 = w[4 as libc::c_int as usize] & t05_6;
        t16_1 = t11_5 | t12_4;
        (*keys)[2 as libc::c_int as usize] = t13_6 ^ t08_6;
        (*keys)[1 as libc::c_int as usize] = t15_3 ^ t16_1;
        (*keys)[0 as libc::c_int as usize] = !t14_4;
        keys = keys.offset(1);
        keys;
    }
    if keys == ((*ctx).keys).as_mut_ptr().offset(33 as libc::c_int as isize) {} else {
        __assert_fail(
            b"keys == ctx->keys + 33\0" as *const u8 as *const libc::c_char,
            b"serpent-set-key.c\0" as *const u8 as *const libc::c_char,
            358 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_serpent_set_key(struct serpent_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_254: {
        if keys == ((*ctx).keys).as_mut_ptr().offset(33 as libc::c_int as isize)
        {} else {
            __assert_fail(
                b"keys == ctx->keys + 33\0" as *const u8 as *const libc::c_char,
                b"serpent-set-key.c\0" as *const u8 as *const libc::c_char,
                358 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_serpent_set_key(struct serpent_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn nettle_serpent128_set_key(
    mut ctx: *mut serpent_ctx,
    mut key: *const uint8_t,
) {
    nettle_serpent_set_key(ctx, 16 as libc::c_int as size_t, key);
}
pub unsafe extern "C" fn nettle_serpent192_set_key(
    mut ctx: *mut serpent_ctx,
    mut key: *const uint8_t,
) {
    nettle_serpent_set_key(ctx, 24 as libc::c_int as size_t, key);
}
pub unsafe extern "C" fn nettle_serpent256_set_key(
    mut ctx: *mut serpent_ctx,
    mut key: *const uint8_t,
) {
    nettle_serpent_set_key(ctx, 32 as libc::c_int as size_t, key);
}
