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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arcfour_ctx {
    pub S: [uint8_t; 256],
    pub i: uint8_t,
    pub j: uint8_t,
}
pub unsafe extern "C" fn nettle_arcfour_set_key(
    mut ctx: *mut arcfour_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    if length >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length >= ARCFOUR_MIN_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"arcfour.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_432: {
        if length >= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length >= ARCFOUR_MIN_KEY_SIZE\0" as *const u8 as *const libc::c_char,
                b"arcfour.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 256 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= ARCFOUR_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"arcfour.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_389: {
        if length <= 256 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= ARCFOUR_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
                b"arcfour.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        (*ctx).S[i as usize] = i as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    k = 0 as libc::c_int as libc::c_uint;
    j = k;
    i = j;
    while i < 256 as libc::c_int as libc::c_uint {
        j = j
            .wrapping_add(
                ((*ctx).S[i as usize] as libc::c_int
                    + *key.offset(k as isize) as libc::c_int) as libc::c_uint,
            );
        j &= 0xff as libc::c_int as libc::c_uint;
        let mut _t: libc::c_int = (*ctx).S[i as usize] as libc::c_int;
        (*ctx).S[i as usize] = (*ctx).S[j as usize];
        (*ctx).S[j as usize] = _t as uint8_t;
        k = (k.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_rem(length) as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    (*ctx).j = 0 as libc::c_int as uint8_t;
    (*ctx).i = (*ctx).j;
}
pub unsafe extern "C" fn nettle_arcfour128_set_key(
    mut ctx: *mut arcfour_ctx,
    mut key: *const uint8_t,
) {
    nettle_arcfour_set_key(ctx, 16 as libc::c_int as size_t, key);
}
pub unsafe extern "C" fn nettle_arcfour_crypt(
    mut ctx: *mut arcfour_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut i: uint8_t = 0;
    let mut j: uint8_t = 0;
    let mut si: libc::c_int = 0;
    let mut sj: libc::c_int = 0;
    i = (*ctx).i;
    j = (*ctx).j;
    loop {
        let fresh0 = length;
        length = length.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        i = i.wrapping_add(1);
        i;
        i = (i as libc::c_int & 0xff as libc::c_int) as uint8_t;
        si = (*ctx).S[i as usize] as libc::c_int;
        j = (j as libc::c_int + si) as uint8_t;
        j = (j as libc::c_int & 0xff as libc::c_int) as uint8_t;
        (*ctx).S[i as usize] = (*ctx).S[j as usize];
        sj = (*ctx).S[i as usize] as libc::c_int;
        (*ctx).S[j as usize] = si as uint8_t;
        let fresh1 = src;
        src = src.offset(1);
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = (*fresh1 as libc::c_int
            ^ (*ctx).S[(si + sj & 0xff as libc::c_int) as usize] as libc::c_int)
            as uint8_t;
    }
    (*ctx).i = i;
    (*ctx).j = j;
}
