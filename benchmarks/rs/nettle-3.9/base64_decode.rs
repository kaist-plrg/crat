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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
pub unsafe extern "C" fn nettle_base64_decode_init(mut ctx: *mut base64_decode_ctx) {
    static mut base64_decode_table: [libc::c_schar; 256] = [
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
        62 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        63 as libc::c_int as libc::c_schar,
        52 as libc::c_int as libc::c_schar,
        53 as libc::c_int as libc::c_schar,
        54 as libc::c_int as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        56 as libc::c_int as libc::c_schar,
        57 as libc::c_int as libc::c_schar,
        58 as libc::c_int as libc::c_schar,
        59 as libc::c_int as libc::c_schar,
        60 as libc::c_int as libc::c_schar,
        61 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
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
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
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
    (*ctx).padding = 0 as libc::c_int as libc::c_uchar;
    (*ctx).bits = (*ctx).padding;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).table = base64_decode_table.as_ptr();
}
pub unsafe extern "C" fn nettle_base64_decode_single(
    mut ctx: *mut base64_decode_ctx,
    mut dst: *mut uint8_t,
    mut src: libc::c_char,
) -> libc::c_int {
    let mut data: libc::c_int = *((*ctx).table).offset(src as uint8_t as isize)
        as libc::c_int;
    match data {
        -1 => return -(1 as libc::c_int),
        -2 => return 0 as libc::c_int,
        -3 => {
            if (*ctx).bits == 0 || (*ctx).padding as libc::c_int > 2 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if (*ctx).word as libc::c_int
                & ((1 as libc::c_int) << (*ctx).bits as libc::c_int) - 1 as libc::c_int
                != 0
            {
                return -(1 as libc::c_int);
            }
            (*ctx).padding = ((*ctx).padding).wrapping_add(1);
            (*ctx).padding;
            (*ctx)
                .bits = ((*ctx).bits as libc::c_int - 2 as libc::c_int) as libc::c_uchar;
            return 0 as libc::c_int;
        }
        _ => {
            if data >= 0 as libc::c_int && data < 0x40 as libc::c_int {} else {
                __assert_fail(
                    b"data >= 0 && data < 0x40\0" as *const u8 as *const libc::c_char,
                    b"base64-decode.c\0" as *const u8 as *const libc::c_char,
                    83 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"int nettle_base64_decode_single(struct base64_decode_ctx *, uint8_t *, char)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2240: {
                if data >= 0 as libc::c_int && data < 0x40 as libc::c_int {} else {
                    __assert_fail(
                        b"data >= 0 && data < 0x40\0" as *const u8
                            as *const libc::c_char,
                        b"base64-decode.c\0" as *const u8 as *const libc::c_char,
                        83 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 77],
                            &[libc::c_char; 77],
                        >(
                            b"int nettle_base64_decode_single(struct base64_decode_ctx *, uint8_t *, char)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*ctx).padding != 0 {
                return -(1 as libc::c_int);
            }
            (*ctx)
                .word = (((*ctx).word as libc::c_int) << 6 as libc::c_int | data)
                as libc::c_ushort;
            (*ctx)
                .bits = ((*ctx).bits as libc::c_int + 6 as libc::c_int) as libc::c_uchar;
            if (*ctx).bits as libc::c_int >= 8 as libc::c_int {
                (*ctx)
                    .bits = ((*ctx).bits as libc::c_int - 8 as libc::c_int)
                    as libc::c_uchar;
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = ((*ctx).word as libc::c_int >> (*ctx).bits as libc::c_int)
                    as uint8_t;
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
    };
}
pub unsafe extern "C" fn nettle_base64_decode_update(
    mut ctx: *mut base64_decode_ctx,
    mut dst_length: *mut size_t,
    mut dst: *mut uint8_t,
    mut src_length: size_t,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut done: size_t = 0;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    done = 0 as libc::c_int as size_t;
    while i < src_length {
        match nettle_base64_decode_single(
            ctx,
            dst.offset(done as isize),
            *src.offset(i as isize),
        ) {
            -1 => return 0 as libc::c_int,
            1 => {
                done = done.wrapping_add(1);
                done;
            }
            0 => {}
            _ => {
                abort();
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if done
        <= src_length
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(6 as libc::c_int as libc::c_ulong)
            .wrapping_div(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"done <= BASE64_DECODE_LENGTH(src_length)\0" as *const u8
                as *const libc::c_char,
            b"base64-decode.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"int nettle_base64_decode_update(struct base64_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2314: {
        if done
            <= src_length
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(6 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
        {} else {
            __assert_fail(
                b"done <= BASE64_DECODE_LENGTH(src_length)\0" as *const u8
                    as *const libc::c_char,
                b"base64-decode.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"int nettle_base64_decode_update(struct base64_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *dst_length = done;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_base64_decode_final(
    mut ctx: *mut base64_decode_ctx,
) -> libc::c_int {
    return ((*ctx).bits as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
