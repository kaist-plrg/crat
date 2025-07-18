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
pub struct base16_decode_ctx {
    pub word: libc::c_uchar,
    pub bits: libc::c_uchar,
}
pub unsafe extern "C" fn nettle_base16_decode_init(mut ctx: *mut base16_decode_ctx) {
    (*ctx).bits = 0 as libc::c_int as libc::c_uchar;
    (*ctx).word = (*ctx).bits;
}
static mut hex_decode_table: [libc::c_schar; 128] = [
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
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
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
pub unsafe extern "C" fn nettle_base16_decode_single(
    mut ctx: *mut base16_decode_ctx,
    mut dst: *mut uint8_t,
    mut src: libc::c_char,
) -> libc::c_int {
    let mut usrc: libc::c_uchar = src as libc::c_uchar;
    let mut digit: libc::c_int = 0;
    if usrc as libc::c_int >= 0x80 as libc::c_int {
        return -(1 as libc::c_int);
    }
    digit = hex_decode_table[usrc as usize] as libc::c_int;
    match digit {
        -1 => return -(1 as libc::c_int),
        -2 => return 0 as libc::c_int,
        _ => {
            if digit >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"digit >= 0\0" as *const u8 as *const libc::c_char,
                    b"base16-decode.c\0" as *const u8 as *const libc::c_char,
                    86 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1435: {
                if digit >= 0 as libc::c_int {} else {
                    __assert_fail(
                        b"digit >= 0\0" as *const u8 as *const libc::c_char,
                        b"base16-decode.c\0" as *const u8 as *const libc::c_char,
                        86 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 77],
                            &[libc::c_char; 77],
                        >(
                            b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if digit < 0x10 as libc::c_int {} else {
                __assert_fail(
                    b"digit < 0x10\0" as *const u8 as *const libc::c_char,
                    b"base16-decode.c\0" as *const u8 as *const libc::c_char,
                    87 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1393: {
                if digit < 0x10 as libc::c_int {} else {
                    __assert_fail(
                        b"digit < 0x10\0" as *const u8 as *const libc::c_char,
                        b"base16-decode.c\0" as *const u8 as *const libc::c_char,
                        87 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 77],
                            &[libc::c_char; 77],
                        >(
                            b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*ctx).bits != 0 {
                *dst = (((*ctx).word as libc::c_int) << 4 as libc::c_int | digit)
                    as uint8_t;
                (*ctx).bits = 0 as libc::c_int as libc::c_uchar;
                return 1 as libc::c_int;
            } else {
                (*ctx).word = digit as libc::c_uchar;
                (*ctx).bits = 4 as libc::c_int as libc::c_uchar;
                return 0 as libc::c_int;
            }
        }
    };
}
pub unsafe extern "C" fn nettle_base16_decode_update(
    mut ctx: *mut base16_decode_ctx,
    mut dst_length: *mut size_t,
    mut dst: *mut uint8_t,
    mut src_length: size_t,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut done: size_t = 0;
    let mut i: size_t = 0;
    done = 0 as libc::c_int as size_t;
    i = done;
    while i < src_length {
        match nettle_base16_decode_single(
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
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"done <= BASE16_DECODE_LENGTH(src_length)\0" as *const u8
                as *const libc::c_char,
            b"base16-decode.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"int nettle_base16_decode_update(struct base16_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1894: {
        if done
            <= src_length
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {} else {
            __assert_fail(
                b"done <= BASE16_DECODE_LENGTH(src_length)\0" as *const u8
                    as *const libc::c_char,
                b"base16-decode.c\0" as *const u8 as *const libc::c_char,
                128 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"int nettle_base16_decode_update(struct base16_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *dst_length = done;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_base16_decode_final(
    mut ctx: *mut base16_decode_ctx,
) -> libc::c_int {
    return ((*ctx).bits as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
