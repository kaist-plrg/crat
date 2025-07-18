use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const libc::c_char,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
}
unsafe extern "C" fn encode_raw(
    mut alphabet: *const libc::c_char,
    mut dst: *mut libc::c_char,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    let mut in_0: *const uint8_t = src.offset(length as isize);
    let mut out: *mut libc::c_char = dst
        .offset(
            length
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_div(3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong) as isize,
        );
    let mut left_over: libc::c_uint = length
        .wrapping_rem(3 as libc::c_int as libc::c_ulong) as libc::c_uint;
    if left_over != 0 {
        in_0 = in_0.offset(-(left_over as isize));
        out = out.offset(-1);
        *out = '=' as i32 as libc::c_char;
        match left_over {
            1 => {
                out = out.offset(-1);
                *out = '=' as i32 as libc::c_char;
                out = out.offset(-1);
                *out = *alphabet
                    .offset(
                        (0x3f as libc::c_int
                            & (*in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                                << 4 as libc::c_int) as isize,
                    );
            }
            2 => {
                out = out.offset(-1);
                *out = *alphabet
                    .offset(
                        (0x3f as libc::c_int
                            & (*in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                                << 2 as libc::c_int) as isize,
                    );
                out = out.offset(-1);
                *out = *alphabet
                    .offset(
                        (0x3f as libc::c_int
                            & ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                                << 4 as libc::c_int
                                | *in_0.offset(1 as libc::c_int as isize) as libc::c_int
                                    >> 4 as libc::c_int)) as isize,
                    );
            }
            _ => {
                abort();
            }
        }
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as libc::c_int
                    & *in_0.offset(0 as libc::c_int as isize) as libc::c_int
                        >> 2 as libc::c_int) as isize,
            );
    }
    while in_0 > src {
        in_0 = in_0.offset(-(3 as libc::c_int as isize));
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as libc::c_int
                    & *in_0.offset(2 as libc::c_int as isize) as libc::c_int) as isize,
            );
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as libc::c_int
                    & ((*in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 2 as libc::c_int
                        | *in_0.offset(2 as libc::c_int as isize) as libc::c_int
                            >> 6 as libc::c_int)) as isize,
            );
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as libc::c_int
                    & ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                        << 4 as libc::c_int
                        | *in_0.offset(1 as libc::c_int as isize) as libc::c_int
                            >> 4 as libc::c_int)) as isize,
            );
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as libc::c_int
                    & *in_0.offset(0 as libc::c_int as isize) as libc::c_int
                        >> 2 as libc::c_int) as isize,
            );
    }
    if in_0 == src {} else {
        __assert_fail(
            b"in == src\0" as *const u8 as *const libc::c_char,
            b"base64-encode.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1707: {
        if in_0 == src {} else {
            __assert_fail(
                b"in == src\0" as *const u8 as *const libc::c_char,
                b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if out == dst {} else {
        __assert_fail(
            b"out == dst\0" as *const u8 as *const libc::c_char,
            b"base64-encode.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1648: {
        if out == dst {} else {
            __assert_fail(
                b"out == dst\0" as *const u8 as *const libc::c_char,
                b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
}
static mut base64_encode_table: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &[libc::c_char; 64],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
};
pub unsafe extern "C" fn nettle_base64_encode_raw(
    mut dst: *mut libc::c_char,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    encode_raw(base64_encode_table.as_ptr(), dst, length, src);
}
pub unsafe extern "C" fn nettle_base64_encode_group(
    mut dst: *mut libc::c_char,
    mut group: uint32_t,
) {
    let fresh0 = dst;
    dst = dst.offset(1);
    *fresh0 = base64_encode_table[(0x3f as libc::c_int as libc::c_uint
        & group >> 18 as libc::c_int) as usize];
    let fresh1 = dst;
    dst = dst.offset(1);
    *fresh1 = base64_encode_table[(0x3f as libc::c_int as libc::c_uint
        & group >> 12 as libc::c_int) as usize];
    let fresh2 = dst;
    dst = dst.offset(1);
    *fresh2 = base64_encode_table[(0x3f as libc::c_int as libc::c_uint
        & group >> 6 as libc::c_int) as usize];
    let fresh3 = dst;
    dst = dst.offset(1);
    *fresh3 = base64_encode_table[(0x3f as libc::c_int as libc::c_uint & group)
        as usize];
}
pub unsafe extern "C" fn nettle_base64_encode_init(mut ctx: *mut base64_encode_ctx) {
    (*ctx).bits = 0 as libc::c_int as libc::c_uchar;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).alphabet = base64_encode_table.as_ptr();
}
pub unsafe extern "C" fn nettle_base64_encode_single(
    mut ctx: *mut base64_encode_ctx,
    mut dst: *mut libc::c_char,
    mut src: uint8_t,
) -> size_t {
    let mut done: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut word: libc::c_uint = (((*ctx).word as libc::c_int) << 8 as libc::c_int
        | src as libc::c_int) as libc::c_uint;
    let mut bits: libc::c_uint = ((*ctx).bits as libc::c_int + 8 as libc::c_int)
        as libc::c_uint;
    while bits >= 6 as libc::c_int as libc::c_uint {
        bits = bits.wrapping_sub(6 as libc::c_int as libc::c_uint);
        let fresh4 = done;
        done = done.wrapping_add(1);
        *dst
            .offset(
                fresh4 as isize,
            ) = *((*ctx).alphabet)
            .offset((0x3f as libc::c_int as libc::c_uint & word >> bits) as isize);
    }
    (*ctx).bits = bits as libc::c_uchar;
    (*ctx).word = word as libc::c_ushort;
    if done <= 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"done <= 2\0" as *const u8 as *const libc::c_char,
            b"base64-encode.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"size_t nettle_base64_encode_single(struct base64_encode_ctx *, char *, uint8_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1350: {
        if done <= 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"done <= 2\0" as *const u8 as *const libc::c_char,
                b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"size_t nettle_base64_encode_single(struct base64_encode_ctx *, char *, uint8_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return done as size_t;
}
pub unsafe extern "C" fn nettle_base64_encode_update(
    mut ctx: *mut base64_encode_ctx,
    mut dst: *mut libc::c_char,
    mut length: size_t,
    mut src: *const uint8_t,
) -> size_t {
    let mut done: size_t = 0 as libc::c_int as size_t;
    let mut left: size_t = length;
    let mut left_over: libc::c_uint = 0;
    let mut bulk: size_t = 0;
    while (*ctx).bits as libc::c_int != 0 && left != 0 {
        left = left.wrapping_sub(1);
        left;
        let fresh5 = src;
        src = src.offset(1);
        done = (done as libc::c_ulong)
            .wrapping_add(
                nettle_base64_encode_single(ctx, dst.offset(done as isize), *fresh5),
            ) as size_t as size_t;
    }
    left_over = left.wrapping_rem(3 as libc::c_int as libc::c_ulong) as libc::c_uint;
    bulk = left.wrapping_sub(left_over as libc::c_ulong);
    if bulk != 0 {
        if bulk.wrapping_rem(3 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!(bulk % 3)\0" as *const u8 as *const libc::c_char,
                b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                161 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2025: {
            if bulk.wrapping_rem(3 as libc::c_int as libc::c_ulong) == 0 {} else {
                __assert_fail(
                    b"!(bulk % 3)\0" as *const u8 as *const libc::c_char,
                    b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                    161 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        encode_raw((*ctx).alphabet, dst.offset(done as isize), bulk, src);
        done = (done as libc::c_ulong)
            .wrapping_add(
                bulk
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_div(3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(4 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        src = src.offset(bulk as isize);
        left = left_over as size_t;
    }
    while left != 0 {
        left = left.wrapping_sub(1);
        left;
        let fresh6 = src;
        src = src.offset(1);
        done = (done as libc::c_ulong)
            .wrapping_add(
                nettle_base64_encode_single(ctx, dst.offset(done as isize), *fresh6),
            ) as size_t as size_t;
    }
    if done
        <= length
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_div(6 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"done <= BASE64_ENCODE_LENGTH(length)\0" as *const u8
                as *const libc::c_char,
            b"base64-encode.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1489: {
        if done
            <= length
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_div(6 as libc::c_int as libc::c_ulong)
        {} else {
            __assert_fail(
                b"done <= BASE64_ENCODE_LENGTH(length)\0" as *const u8
                    as *const libc::c_char,
                b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return done;
}
pub unsafe extern "C" fn nettle_base64_encode_final(
    mut ctx: *mut base64_encode_ctx,
    mut dst: *mut libc::c_char,
) -> size_t {
    let mut done: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut bits: libc::c_uint = (*ctx).bits as libc::c_uint;
    if bits != 0 {
        let fresh7 = done;
        done = done.wrapping_add(1);
        *dst
            .offset(
                fresh7 as isize,
            ) = *((*ctx).alphabet)
            .offset(
                (0x3f as libc::c_int
                    & ((*ctx).word as libc::c_int)
                        << 6 as libc::c_int - (*ctx).bits as libc::c_int) as isize,
            );
        while bits < 6 as libc::c_int as libc::c_uint {
            let fresh8 = done;
            done = done.wrapping_add(1);
            *dst.offset(fresh8 as isize) = '=' as i32 as libc::c_char;
            bits = bits.wrapping_add(2 as libc::c_int as libc::c_uint);
        }
        (*ctx).bits = 0 as libc::c_int as libc::c_uchar;
    }
    if done <= 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"done <= BASE64_ENCODE_FINAL_LENGTH\0" as *const u8 as *const libc::c_char,
            b"base64-encode.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"size_t nettle_base64_encode_final(struct base64_encode_ctx *, char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2130: {
        if done <= 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"done <= BASE64_ENCODE_FINAL_LENGTH\0" as *const u8
                    as *const libc::c_char,
                b"base64-encode.c\0" as *const u8 as *const libc::c_char,
                198 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"size_t nettle_base64_encode_final(struct base64_encode_ctx *, char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return done as size_t;
}
