use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_mpz_sizeinbase_256_u(x: *const __mpz_struct) -> size_t;
    fn nettle_mpz_get_str_256(length: size_t, s: *mut uint8_t, x: *const __mpz_struct);
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
    fn nettle_base64_encode_group(dst: *mut libc::c_char, group: uint32_t);
    fn nettle_buffer_grow(buffer: *mut nettle_buffer, length: size_t) -> libc::c_int;
    fn nettle_buffer_write(
        buffer: *mut nettle_buffer,
        length: size_t,
        data: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_buffer_space(buffer: *mut nettle_buffer, length: size_t) -> *mut uint8_t;
    fn nettle_rsa_sha1_sign(
        key: *const rsa_private_key,
        hash: *mut sha1_ctx,
        signature: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_sha1_update(ctx: *mut sha1_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha1_digest(ctx: *mut sha1_ctx, length: size_t, digest: *mut uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_private_key {
    pub size: size_t,
    pub d: mpz_t,
    pub p: mpz_t,
    pub q: mpz_t,
    pub a: mpz_t,
    pub b: mpz_t,
    pub c: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
pub const PGP_LENGTH_FOUR_OCTETS: pgp_lengths = 8384;
pub const PGP_LENGTH_TWO_OCTETS: pgp_lengths = 192;
pub const PGP_RSA: pgp_public_key_algorithm = 1;
pub const PGP_TAG_PUBLIC_KEY: pgp_tag = 6;
pub const PGP_SUBPACKET_ISSUER_KEY_ID: pgp_subpacket_tag = 16;
pub const PGP_SHA1: pgp_hash_algorithm = 2;
pub const PGP_TAG_SIGNATURE: pgp_tag = 2;
pub const PGP_TAG_USERID: pgp_tag = 13;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const libc::c_char,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
}
pub type pgp_lengths = libc::c_uint;
pub const PGP_LENGTH_ONE_OCTET: pgp_lengths = 0;
pub type pgp_public_key_algorithm = libc::c_uint;
pub const PGP_EL_GAMAL: pgp_public_key_algorithm = 20;
pub const PGP_DSA: pgp_public_key_algorithm = 17;
pub const PGP_EL_GAMAL_ENCRYPT: pgp_public_key_algorithm = 16;
pub const PGP_RSA_SIGN: pgp_public_key_algorithm = 3;
pub const PGP_RSA_ENCRYPT: pgp_public_key_algorithm = 2;
pub type pgp_hash_algorithm = libc::c_uint;
pub const PGP_HAVAL: pgp_hash_algorithm = 7;
pub const PGP_TIGER192: pgp_hash_algorithm = 6;
pub const PGP_MD2: pgp_hash_algorithm = 5;
pub const PGP_RIPEMD: pgp_hash_algorithm = 3;
pub const PGP_MD5: pgp_hash_algorithm = 1;
pub type pgp_tag = libc::c_uint;
pub const PGP_TAG_PUBLIC_SUBKEY: pgp_tag = 14;
pub const PGP_TAG_TRUST: pgp_tag = 12;
pub const PGP_TAG_LITERAL: pgp_tag = 11;
pub const PGP_TAG_MARKER: pgp_tag = 10;
pub const PGP_TAG_ENCRYPTED: pgp_tag = 9;
pub const PGP_TAG_COMPRESSED: pgp_tag = 8;
pub const PGP_TAG_SECRET_SUBKEY: pgp_tag = 7;
pub const PGP_TAG_SECRET_KEY: pgp_tag = 5;
pub const PGP_TAG_ONE_PASS_SIGNATURE: pgp_tag = 4;
pub const PGP_TAG_SYMMETRIC_SESSION_KEY: pgp_tag = 3;
pub const PGP_TAG_PUBLIC_SESSION_KEY: pgp_tag = 1;
pub type pgp_subpacket_tag = libc::c_uint;
pub const PGP_SUBPACKET_REASON_FOR_REVOCATION: pgp_subpacket_tag = 29;
pub const PGP_SUBPACKET_SIGNERS_USER_ID: pgp_subpacket_tag = 28;
pub const PGP_SUBPACKET_KEY_FLAGS: pgp_subpacket_tag = 27;
pub const PGP_SUBPACKET_POLICY_URL: pgp_subpacket_tag = 26;
pub const PGP_SUBPACKET_PRIMARY_USER_ID: pgp_subpacket_tag = 25;
pub const PGP_SUBPACKET_PREFERRED_KEY_SERVER: pgp_subpacket_tag = 24;
pub const PGP_SUBPACKET_KEY_SERVER_PREFERENCES: pgp_subpacket_tag = 23;
pub const PGP_SUBPACKET_PREFERRED_COMPRESSION_ALGORITHMS: pgp_subpacket_tag = 22;
pub const PGP_SUBPACKET_PREFERRED_HASH_ALGORITHMS: pgp_subpacket_tag = 21;
pub const PGP_SUBPACKET_NOTATION_DATA: pgp_subpacket_tag = 20;
pub const PGP_SUBPACKET_REVOCATION_KEY: pgp_subpacket_tag = 12;
pub const PGP_SUBPACKET_PREFERRED_SYMMETRIC_ALGORITHMS: pgp_subpacket_tag = 11;
pub const PGP_SUBPACKET_PLACEHOLDER: pgp_subpacket_tag = 10;
pub const PGP_SUBPACKET_KEY_EXPIRATION_TIME: pgp_subpacket_tag = 9;
pub const PGP_SUBPACKET_REVOCABLE: pgp_subpacket_tag = 7;
pub const PGP_SUBPACKET_REGULAR_EXPRESSION: pgp_subpacket_tag = 6;
pub const PGP_SUBPACKET_TRUST_SIGNATURE: pgp_subpacket_tag = 5;
pub const PGP_SUBPACKET_EXPORTABLE_CERTIFICATION: pgp_subpacket_tag = 4;
pub const PGP_SUBPACKET_SIGNATURE_EXPIRATION_TIME: pgp_subpacket_tag = 3;
pub const PGP_SUBPACKET_CREATION_TIME: pgp_subpacket_tag = 2;
pub unsafe extern "C" fn nettle_pgp_put_uint32(
    mut buffer: *mut nettle_buffer,
    mut i: uint32_t,
) -> libc::c_int {
    let mut p: *mut uint8_t = nettle_buffer_space(buffer, 4 as libc::c_int as size_t);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    *p
        .offset(
            0 as libc::c_int as isize,
        ) = (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *p
        .offset(
            1 as libc::c_int as isize,
        ) = (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *p
        .offset(
            2 as libc::c_int as isize,
        ) = (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *p
        .offset(
            3 as libc::c_int as isize,
        ) = (i & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_put_uint16(
    mut buffer: *mut nettle_buffer,
    mut i: libc::c_uint,
) -> libc::c_int {
    let mut p: *mut uint8_t = nettle_buffer_space(buffer, 2 as libc::c_int as size_t);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    *p
        .offset(
            0 as libc::c_int as isize,
        ) = (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *p
        .offset(
            1 as libc::c_int as isize,
        ) = (i & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_put_mpi(
    mut buffer: *mut nettle_buffer,
    mut x: *const __mpz_struct,
) -> libc::c_int {
    let mut bits: libc::c_uint = __gmpz_sizeinbase(x, 2 as libc::c_int) as libc::c_uint;
    let mut octets: libc::c_uint = bits
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    if nettle_pgp_put_uint16(buffer, bits) == 0 {
        return 0 as libc::c_int;
    }
    p = nettle_buffer_space(buffer, octets as size_t);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    nettle_mpz_get_str_256(octets as size_t, p, x);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_put_string(
    mut buffer: *mut nettle_buffer,
    mut length: libc::c_uint,
    mut s: *const uint8_t,
) -> libc::c_int {
    return nettle_buffer_write(buffer, length as size_t, s);
}
pub unsafe extern "C" fn nettle_pgp_put_length(
    mut buffer: *mut nettle_buffer,
    mut length: libc::c_uint,
) -> libc::c_int {
    if length < PGP_LENGTH_TWO_OCTETS as libc::c_int as libc::c_uint {
        return (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh0 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh0 as isize) = length as uint8_t;
                1 as libc::c_int != 0
            }) as libc::c_int
    } else if length < PGP_LENGTH_FOUR_OCTETS as libc::c_int as libc::c_uint {
        return nettle_pgp_put_uint16(
            buffer,
            length
                .wrapping_add((192 as libc::c_int * 255 as libc::c_int) as libc::c_uint),
        )
    } else {
        return (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh1 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents)
                    .offset(fresh1 as isize) = 0xff as libc::c_int as uint8_t;
                1 as libc::c_int != 0
            } && nettle_pgp_put_uint32(buffer, length) != 0) as libc::c_int
    };
}
pub unsafe extern "C" fn nettle_pgp_put_header(
    mut buffer: *mut nettle_buffer,
    mut tag: libc::c_uint,
    mut length: libc::c_uint,
) -> libc::c_int {
    if tag < 0x40 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tag < 0x40\0" as *const u8 as *const libc::c_char,
            b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"int nettle_pgp_put_header(struct nettle_buffer *, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5275: {
        if tag < 0x40 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"tag < 0x40\0" as *const u8 as *const libc::c_char,
                b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"int nettle_pgp_put_header(struct nettle_buffer *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return (((*buffer).size < (*buffer).alloc
        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
        && {
            let fresh2 = (*buffer).size;
            (*buffer).size = ((*buffer).size).wrapping_add(1);
            *((*buffer).contents)
                .offset(
                    fresh2 as isize,
                ) = (0xc0 as libc::c_int as libc::c_uint | tag) as uint8_t;
            1 as libc::c_int != 0
        } && nettle_pgp_put_length(buffer, length) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_put_header_length(
    mut buffer: *mut nettle_buffer,
    mut start: libc::c_uint,
    mut field_size: libc::c_uint,
) {
    let mut length: libc::c_uint = 0;
    match field_size {
        1 => {
            length = ((*buffer).size)
                .wrapping_sub(
                    start.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ) as libc::c_uint;
            if length < PGP_LENGTH_TWO_OCTETS as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"length < PGP_LENGTH_TWO_OCTETS\0" as *const u8
                        as *const libc::c_char,
                    b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                    156 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"void nettle_pgp_put_header_length(struct nettle_buffer *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5604: {
                if length < PGP_LENGTH_TWO_OCTETS as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"length < PGP_LENGTH_TWO_OCTETS\0" as *const u8
                            as *const libc::c_char,
                        b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                        156 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 86],
                            &[libc::c_char; 86],
                        >(
                            b"void nettle_pgp_put_header_length(struct nettle_buffer *, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            *((*buffer).contents)
                .offset(
                    start.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = length as uint8_t;
        }
        2 => {
            length = ((*buffer).size)
                .wrapping_sub(
                    start.wrapping_add(3 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ) as libc::c_uint;
            if length < PGP_LENGTH_FOUR_OCTETS as libc::c_int as libc::c_uint
                && length >= PGP_LENGTH_TWO_OCTETS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"length < PGP_LENGTH_FOUR_OCTETS && length >= PGP_LENGTH_TWO_OCTETS\0"
                        as *const u8 as *const libc::c_char,
                    b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"void nettle_pgp_put_header_length(struct nettle_buffer *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5519: {
                if length < PGP_LENGTH_FOUR_OCTETS as libc::c_int as libc::c_uint
                    && length >= PGP_LENGTH_TWO_OCTETS as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"length < PGP_LENGTH_FOUR_OCTETS && length >= PGP_LENGTH_TWO_OCTETS\0"
                            as *const u8 as *const libc::c_char,
                        b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                        162 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 86],
                            &[libc::c_char; 86],
                        >(
                            b"void nettle_pgp_put_header_length(struct nettle_buffer *, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            *((*buffer).contents)
                .offset(start as isize)
                .offset(1 as libc::c_int as isize)
                .offset(
                    0 as libc::c_int as isize,
                ) = (length
                .wrapping_add((192 as libc::c_int * 255 as libc::c_int) as libc::c_uint)
                >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
            *((*buffer).contents)
                .offset(start as isize)
                .offset(1 as libc::c_int as isize)
                .offset(
                    1 as libc::c_int as isize,
                ) = (length
                .wrapping_add((192 as libc::c_int * 255 as libc::c_int) as libc::c_uint)
                & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        }
        4 => {
            length = ((*buffer).size)
                .wrapping_sub(
                    start.wrapping_add(5 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ) as libc::c_uint;
            *((*buffer).contents)
                .offset(start as isize)
                .offset(2 as libc::c_int as isize)
                .offset(
                    0 as libc::c_int as isize,
                ) = (length >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t;
            *((*buffer).contents)
                .offset(start as isize)
                .offset(2 as libc::c_int as isize)
                .offset(
                    1 as libc::c_int as isize,
                ) = (length >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t;
            *((*buffer).contents)
                .offset(start as isize)
                .offset(2 as libc::c_int as isize)
                .offset(
                    2 as libc::c_int as isize,
                ) = (length >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t;
            *((*buffer).contents)
                .offset(start as isize)
                .offset(2 as libc::c_int as isize)
                .offset(
                    3 as libc::c_int as isize,
                ) = (length & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        }
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn nettle_pgp_put_userid(
    mut buffer: *mut nettle_buffer,
    mut length: libc::c_uint,
    mut name: *const uint8_t,
) -> libc::c_int {
    return (nettle_pgp_put_header(
        buffer,
        PGP_TAG_USERID as libc::c_int as libc::c_uint,
        length,
    ) != 0 && nettle_pgp_put_string(buffer, length, name) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_sub_packet_start(
    mut buffer: *mut nettle_buffer,
) -> libc::c_uint {
    return (if !(nettle_buffer_space(buffer, 2 as libc::c_int as size_t)).is_null() {
        (*buffer).size
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_uint;
}
pub unsafe extern "C" fn nettle_pgp_put_sub_packet(
    mut buffer: *mut nettle_buffer,
    mut type_0: libc::c_uint,
    mut length: libc::c_uint,
    mut data: *const uint8_t,
) -> libc::c_int {
    return (nettle_pgp_put_length(
        buffer,
        length.wrapping_add(1 as libc::c_int as libc::c_uint),
    ) != 0
        && (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh3 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh3 as isize) = type_0 as uint8_t;
                1 as libc::c_int != 0
            }) && nettle_pgp_put_string(buffer, length, data) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_sub_packet_end(
    mut buffer: *mut nettle_buffer,
    mut start: libc::c_uint,
) {
    let mut length: libc::c_uint = 0;
    if start >= 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"start >= 2\0" as *const u8 as *const libc::c_char,
            b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void nettle_pgp_sub_packet_end(struct nettle_buffer *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_5921: {
        if start >= 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"start >= 2\0" as *const u8 as *const libc::c_char,
                b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                205 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void nettle_pgp_sub_packet_end(struct nettle_buffer *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if start as libc::c_ulong <= (*buffer).size {} else {
        __assert_fail(
            b"start <= buffer->size\0" as *const u8 as *const libc::c_char,
            b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void nettle_pgp_sub_packet_end(struct nettle_buffer *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_5875: {
        if start as libc::c_ulong <= (*buffer).size {} else {
            __assert_fail(
                b"start <= buffer->size\0" as *const u8 as *const libc::c_char,
                b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void nettle_pgp_sub_packet_end(struct nettle_buffer *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    length = ((*buffer).size).wrapping_sub(start as libc::c_ulong) as libc::c_uint;
    *((*buffer).contents)
        .offset(start as isize)
        .offset(-(2 as libc::c_int as isize))
        .offset(
            0 as libc::c_int as isize,
        ) = (length >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    *((*buffer).contents)
        .offset(start as isize)
        .offset(-(2 as libc::c_int as isize))
        .offset(
            1 as libc::c_int as isize,
        ) = (length >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    *((*buffer).contents)
        .offset(start as isize)
        .offset(-(2 as libc::c_int as isize))
        .offset(
            2 as libc::c_int as isize,
        ) = (length >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    *((*buffer).contents)
        .offset(start as isize)
        .offset(-(2 as libc::c_int as isize))
        .offset(
            3 as libc::c_int as isize,
        ) = (length & 0xff as libc::c_int as libc::c_uint) as uint8_t;
}
pub unsafe extern "C" fn nettle_pgp_put_public_rsa_key(
    mut buffer: *mut nettle_buffer,
    mut pub_0: *const rsa_public_key,
    mut timestamp: time_t,
) -> libc::c_int {
    let mut start: libc::c_uint = 0;
    let mut length: libc::c_uint = 0;
    length = ((4 as libc::c_int * 4 as libc::c_int) as libc::c_ulong)
        .wrapping_add(nettle_mpz_sizeinbase_256_u(((*pub_0).n).as_ptr()))
        .wrapping_add(nettle_mpz_sizeinbase_256_u(((*pub_0).e).as_ptr()))
        as libc::c_uint;
    if nettle_pgp_put_header(
        buffer,
        PGP_TAG_PUBLIC_KEY as libc::c_int as libc::c_uint,
        length,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    start = (*buffer).size as libc::c_uint;
    if !(nettle_pgp_put_header(
        buffer,
        PGP_TAG_PUBLIC_KEY as libc::c_int as libc::c_uint,
        PGP_LENGTH_TWO_OCTETS as libc::c_int as libc::c_uint,
    ) != 0 && nettle_pgp_put_uint32(buffer, 4 as libc::c_int as uint32_t) != 0
        && nettle_pgp_put_uint32(buffer, timestamp as uint32_t) != 0
        && nettle_pgp_put_uint32(buffer, PGP_RSA as libc::c_int as uint32_t) != 0
        && nettle_pgp_put_mpi(buffer, ((*pub_0).n).as_ptr()) != 0
        && nettle_pgp_put_mpi(buffer, ((*pub_0).e).as_ptr()) != 0)
    {
        return 0 as libc::c_int;
    }
    if (*buffer).size == start.wrapping_add(length) as libc::c_ulong {} else {
        __assert_fail(
            b"buffer->size == start + length\0" as *const u8 as *const libc::c_char,
            b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"int nettle_pgp_put_public_rsa_key(struct nettle_buffer *, const struct rsa_public_key *, time_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5977: {
        if (*buffer).size == start.wrapping_add(length) as libc::c_ulong {} else {
            __assert_fail(
                b"buffer->size == start + length\0" as *const u8 as *const libc::c_char,
                b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                241 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"int nettle_pgp_put_public_rsa_key(struct nettle_buffer *, const struct rsa_public_key *, time_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_put_rsa_sha1_signature(
    mut buffer: *mut nettle_buffer,
    mut key: *const rsa_private_key,
    mut keyid: *const uint8_t,
    mut type_0: libc::c_uint,
    mut hash: *mut sha1_ctx,
) -> libc::c_int {
    let mut signature_start: libc::c_uint = (*buffer).size as libc::c_uint;
    let mut hash_end: libc::c_uint = 0;
    let mut sub_packet_start: libc::c_uint = 0;
    let mut trailer: [uint8_t; 6] = [0; 6];
    let mut s: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if !(nettle_pgp_put_header(
        buffer,
        PGP_TAG_SIGNATURE as libc::c_int as libc::c_uint,
        PGP_LENGTH_FOUR_OCTETS as libc::c_int as libc::c_uint,
    ) != 0
        && (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh4 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents)
                    .offset(fresh4 as isize) = 4 as libc::c_int as uint8_t;
                1 as libc::c_int != 0
            })
        && (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh5 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh5 as isize) = type_0 as uint8_t;
                1 as libc::c_int != 0
            })
        && (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh6 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents)
                    .offset(fresh6 as isize) = PGP_RSA as libc::c_int as uint8_t;
                1 as libc::c_int != 0
            })
        && (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh7 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents)
                    .offset(fresh7 as isize) = PGP_SHA1 as libc::c_int as uint8_t;
                1 as libc::c_int != 0
            }) && nettle_pgp_put_uint16(buffer, 0 as libc::c_int as libc::c_uint) != 0)
    {
        return 0 as libc::c_int;
    }
    hash_end = (*buffer).size as libc::c_uint;
    nettle_sha1_update(
        hash,
        hash_end.wrapping_sub(signature_start) as size_t,
        ((*buffer).contents).offset(signature_start as isize),
    );
    trailer[0 as libc::c_int as usize] = 4 as libc::c_int as uint8_t;
    trailer[1 as libc::c_int as usize] = 0xff as libc::c_int as uint8_t;
    *trailer
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (((*buffer).size).wrapping_sub(signature_start as libc::c_ulong)
        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *trailer
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (((*buffer).size).wrapping_sub(signature_start as libc::c_ulong)
        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *trailer
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (((*buffer).size).wrapping_sub(signature_start as libc::c_ulong)
        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *trailer
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (((*buffer).size).wrapping_sub(signature_start as libc::c_ulong)
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    nettle_sha1_update(
        hash,
        ::std::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong,
        trailer.as_mut_ptr(),
    );
    let mut hcopy: sha1_ctx = *hash;
    let mut p: *mut uint8_t = nettle_buffer_space(buffer, 2 as libc::c_int as size_t);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    nettle_sha1_digest(&mut hcopy, 2 as libc::c_int as size_t, p);
    sub_packet_start = nettle_pgp_sub_packet_start(buffer);
    if sub_packet_start == 0 {
        return 0 as libc::c_int;
    }
    if nettle_pgp_put_sub_packet(
        buffer,
        PGP_SUBPACKET_ISSUER_KEY_ID as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        keyid,
    ) != 0
    {
        nettle_pgp_sub_packet_end(buffer, sub_packet_start);
        return 0 as libc::c_int;
    }
    __gmpz_init(s.as_mut_ptr());
    if !(nettle_rsa_sha1_sign(key, hash, s.as_mut_ptr()) != 0
        && nettle_pgp_put_mpi(buffer, s.as_mut_ptr() as *const __mpz_struct) != 0)
    {
        __gmpz_clear(s.as_mut_ptr());
        return 0 as libc::c_int;
    }
    __gmpz_clear(s.as_mut_ptr());
    nettle_pgp_put_header_length(
        buffer,
        signature_start,
        4 as libc::c_int as libc::c_uint,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_pgp_crc24(
    mut length: libc::c_uint,
    mut data: *const uint8_t,
) -> uint32_t {
    let mut crc: uint32_t = 0xb704ce as libc::c_long as uint32_t;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let mut j: libc::c_uint = 0;
        crc ^= (*data.offset(i as isize) as libc::c_uint) << 16 as libc::c_int;
        j = 0 as libc::c_int as libc::c_uint;
        while j < 8 as libc::c_int as libc::c_uint {
            crc <<= 1 as libc::c_int;
            if crc & 0x1000000 as libc::c_int as libc::c_uint != 0 {
                crc = (crc as libc::c_long ^ 0x1864cfb as libc::c_long) as uint32_t;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if crc < 0x1000000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"crc < 0x1000000\0" as *const u8 as *const libc::c_char,
            b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"uint32_t nettle_pgp_crc24(unsigned int, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6720: {
        if crc < 0x1000000 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"crc < 0x1000000\0" as *const u8 as *const libc::c_char,
                b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                337 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"uint32_t nettle_pgp_crc24(unsigned int, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return crc;
}
unsafe extern "C" fn write_string(
    mut buffer: *mut nettle_buffer,
    mut s: *const libc::c_char,
) -> libc::c_int {
    return nettle_buffer_write(buffer, strlen(s), s as *const uint8_t);
}
pub unsafe extern "C" fn nettle_pgp_armor(
    mut buffer: *mut nettle_buffer,
    mut tag: *const libc::c_char,
    mut length: libc::c_uint,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut ctx: base64_encode_ctx = base64_encode_ctx {
        alphabet: 0 as *const libc::c_char,
        word: 0,
        bits: 0,
    };
    let mut crc: libc::c_uint = nettle_pgp_crc24(length, data);
    nettle_base64_encode_init(&mut ctx);
    if !(write_string(buffer, b"BEGIN PGP \0" as *const u8 as *const libc::c_char) != 0
        && write_string(buffer, tag) != 0
        && write_string(
            buffer,
            b"\nComment: Nettle\n\n\0" as *const u8 as *const libc::c_char,
        ) != 0)
    {
        return 0 as libc::c_int;
    }
    while length >= 45 as libc::c_int as libc::c_uint {
        let mut done: libc::c_uint = 0;
        let mut p: *mut libc::c_char = nettle_buffer_space(
            buffer,
            ((45 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int)
                / 6 as libc::c_int) as size_t,
        ) as *mut libc::c_char;
        if p.is_null() {
            return 0 as libc::c_int;
        }
        done = nettle_base64_encode_update(
            &mut ctx,
            p,
            45 as libc::c_int as size_t,
            data,
        ) as libc::c_uint;
        if done
            <= ((45 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int)
                / 6 as libc::c_int) as libc::c_uint
        {} else {
            __assert_fail(
                b"done <= TEXT_PER_LINE\0" as *const u8 as *const libc::c_char,
                b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                381 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"int nettle_pgp_armor(struct nettle_buffer *, const char *, unsigned int, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7232: {
            if done
                <= ((45 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int)
                    / 6 as libc::c_int) as libc::c_uint
            {} else {
                __assert_fail(
                    b"done <= TEXT_PER_LINE\0" as *const u8 as *const libc::c_char,
                    b"pgp-encode.c\0" as *const u8 as *const libc::c_char,
                    381 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 90],
                        &[libc::c_char; 90],
                    >(
                        b"int nettle_pgp_armor(struct nettle_buffer *, const char *, unsigned int, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*buffer)
            .size = ((*buffer).size as libc::c_ulong)
            .wrapping_sub(
                (((45 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int)
                    / 6 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(done) as libc::c_ulong,
            ) as size_t as size_t;
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh8 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh8 as isize) = '\n' as i32 as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
        length = length.wrapping_sub(45 as libc::c_int as libc::c_uint);
        data = data.offset(45 as libc::c_int as isize);
    }
    if length != 0 {
        let mut text_size: libc::c_uint = length
            .wrapping_mul(8 as libc::c_int as libc::c_uint)
            .wrapping_add(4 as libc::c_int as libc::c_uint)
            .wrapping_div(6 as libc::c_int as libc::c_uint)
            .wrapping_add(3 as libc::c_int as libc::c_uint);
        let mut done_0: libc::c_uint = 0;
        let mut p_0: *mut libc::c_char = nettle_buffer_space(buffer, text_size as size_t)
            as *mut libc::c_char;
        if p_0.is_null() {
            return 0 as libc::c_int;
        }
        done_0 = nettle_base64_encode_update(&mut ctx, p_0, length as size_t, data)
            as libc::c_uint;
        done_0 = (done_0 as libc::c_ulong)
            .wrapping_add(
                nettle_base64_encode_final(&mut ctx, p_0.offset(done_0 as isize)),
            ) as libc::c_uint as libc::c_uint;
        (*buffer)
            .size = ((*buffer).size as libc::c_ulong)
            .wrapping_sub(text_size.wrapping_sub(done_0) as libc::c_ulong) as size_t
            as size_t;
        if !(((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh9 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh9 as isize) = '\n' as i32 as uint8_t;
                1 as libc::c_int != 0
            })
        {
            return 0 as libc::c_int;
        }
    }
    if !(((*buffer).size < (*buffer).alloc
        || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
        && {
            let fresh10 = (*buffer).size;
            (*buffer).size = ((*buffer).size).wrapping_add(1);
            *((*buffer).contents).offset(fresh10 as isize) = '=' as i32 as uint8_t;
            1 as libc::c_int != 0
        })
    {
        return 0 as libc::c_int;
    }
    let mut p_1: *mut libc::c_char = nettle_buffer_space(
        buffer,
        4 as libc::c_int as size_t,
    ) as *mut libc::c_char;
    if p_1.is_null() {
        return 0 as libc::c_int;
    }
    nettle_base64_encode_group(p_1, crc);
    return (write_string(buffer, b"\nBEGIN PGP \0" as *const u8 as *const libc::c_char)
        != 0 && write_string(buffer, tag) != 0
        && (((*buffer).size < (*buffer).alloc
            || nettle_buffer_grow(buffer, 1 as libc::c_int as size_t) != 0)
            && {
                let fresh11 = (*buffer).size;
                (*buffer).size = ((*buffer).size).wrapping_add(1);
                *((*buffer).contents).offset(fresh11 as isize) = '\n' as i32 as uint8_t;
                1 as libc::c_int != 0
            })) as libc::c_int;
}
