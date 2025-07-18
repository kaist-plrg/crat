use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_sexp_iterator_first(
        iterator: *mut sexp_iterator,
        length: size_t,
        input: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_base64_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base64_decode_final(ctx: *mut base64_decode_ctx) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type sexp_type = libc::c_uint;
pub const SEXP_END: sexp_type = 2;
pub const SEXP_LIST: sexp_type = 1;
pub const SEXP_ATOM: sexp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_iterator {
    pub length: size_t,
    pub buffer: *const uint8_t,
    pub start: size_t,
    pub pos: size_t,
    pub level: libc::c_uint,
    pub type_0: sexp_type,
    pub display_length: size_t,
    pub display: *const uint8_t,
    pub atom_length: size_t,
    pub atom: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
pub unsafe extern "C" fn nettle_sexp_transport_iterator_first(
    mut iterator: *mut sexp_iterator,
    mut length: size_t,
    mut input: *mut uint8_t,
) -> libc::c_int {
    let mut in_0: size_t = 0 as libc::c_int as size_t;
    let mut out: size_t = 0 as libc::c_int as size_t;
    while in_0 < length {
        match *input.offset(in_0 as isize) as libc::c_int {
            32 | 9 | 10 | 13 => {
                in_0 = in_0.wrapping_add(1);
                in_0;
            }
            59 => {
                loop {
                    in_0 = in_0.wrapping_add(1);
                    if !(in_0 < length
                        && *input.offset(in_0 as isize) as libc::c_int != '\n' as i32)
                    {
                        break;
                    }
                }
            }
            123 => {
                let mut ctx: base64_decode_ctx = base64_decode_ctx {
                    table: 0 as *const libc::c_schar,
                    word: 0,
                    bits: 0,
                    padding: 0,
                };
                let mut coded_length: size_t = 0;
                let mut end: size_t = 0;
                in_0 = in_0.wrapping_add(1);
                end = in_0;
                while end < length
                    && *input.offset(end as isize) as libc::c_int != '}' as i32
                {
                    end = end.wrapping_add(1);
                    end;
                }
                if end == length {
                    return 0 as libc::c_int;
                }
                nettle_base64_decode_init(&mut ctx);
                if nettle_base64_decode_update(
                    &mut ctx,
                    &mut coded_length,
                    input.offset(out as isize),
                    end.wrapping_sub(in_0),
                    input.offset(in_0 as isize) as *const libc::c_char,
                ) != 0 && nettle_base64_decode_final(&mut ctx) != 0
                {
                    out = (out as libc::c_ulong).wrapping_add(coded_length) as size_t
                        as size_t;
                    in_0 = end.wrapping_add(1 as libc::c_int as libc::c_ulong);
                } else {
                    return 0 as libc::c_int
                }
            }
            _ => {
                break;
            }
        }
    }
    if out == 0 {
        input = input.offset(in_0 as isize);
        length = (length as libc::c_ulong).wrapping_sub(in_0) as size_t as size_t;
    } else if in_0 == length {
        length = out;
    } else if !(out == in_0) {
        if out < in_0 {} else {
            __assert_fail(
                b"out < in\0" as *const u8 as *const libc::c_char,
                b"sexp-transport.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"int nettle_sexp_transport_iterator_first(struct sexp_iterator *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_588: {
            if out < in_0 {} else {
                __assert_fail(
                    b"out < in\0" as *const u8 as *const libc::c_char,
                    b"sexp-transport.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"int nettle_sexp_transport_iterator_first(struct sexp_iterator *, size_t, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        memmove(
            input.offset(out as isize) as *mut libc::c_void,
            input.offset(in_0 as isize) as *const libc::c_void,
            length.wrapping_sub(in_0),
        );
        length = (length as libc::c_ulong).wrapping_sub(in_0.wrapping_sub(out)) as size_t
            as size_t;
    }
    return nettle_sexp_iterator_first(iterator, length, input);
}
