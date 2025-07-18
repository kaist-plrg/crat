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
    fn _nettle_sha256_compress_n(
        state: *mut uint32_t,
        k: *const uint32_t,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_write_be32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
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
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
static mut K: [uint32_t; 64] = [
    0x428a2f98 as libc::c_ulong as uint32_t,
    0x71374491 as libc::c_ulong as uint32_t,
    0xb5c0fbcf as libc::c_ulong as uint32_t,
    0xe9b5dba5 as libc::c_ulong as uint32_t,
    0x3956c25b as libc::c_ulong as uint32_t,
    0x59f111f1 as libc::c_ulong as uint32_t,
    0x923f82a4 as libc::c_ulong as uint32_t,
    0xab1c5ed5 as libc::c_ulong as uint32_t,
    0xd807aa98 as libc::c_ulong as uint32_t,
    0x12835b01 as libc::c_ulong as uint32_t,
    0x243185be as libc::c_ulong as uint32_t,
    0x550c7dc3 as libc::c_ulong as uint32_t,
    0x72be5d74 as libc::c_ulong as uint32_t,
    0x80deb1fe as libc::c_ulong as uint32_t,
    0x9bdc06a7 as libc::c_ulong as uint32_t,
    0xc19bf174 as libc::c_ulong as uint32_t,
    0xe49b69c1 as libc::c_ulong as uint32_t,
    0xefbe4786 as libc::c_ulong as uint32_t,
    0xfc19dc6 as libc::c_ulong as uint32_t,
    0x240ca1cc as libc::c_ulong as uint32_t,
    0x2de92c6f as libc::c_ulong as uint32_t,
    0x4a7484aa as libc::c_ulong as uint32_t,
    0x5cb0a9dc as libc::c_ulong as uint32_t,
    0x76f988da as libc::c_ulong as uint32_t,
    0x983e5152 as libc::c_ulong as uint32_t,
    0xa831c66d as libc::c_ulong as uint32_t,
    0xb00327c8 as libc::c_ulong as uint32_t,
    0xbf597fc7 as libc::c_ulong as uint32_t,
    0xc6e00bf3 as libc::c_ulong as uint32_t,
    0xd5a79147 as libc::c_ulong as uint32_t,
    0x6ca6351 as libc::c_ulong as uint32_t,
    0x14292967 as libc::c_ulong as uint32_t,
    0x27b70a85 as libc::c_ulong as uint32_t,
    0x2e1b2138 as libc::c_ulong as uint32_t,
    0x4d2c6dfc as libc::c_ulong as uint32_t,
    0x53380d13 as libc::c_ulong as uint32_t,
    0x650a7354 as libc::c_ulong as uint32_t,
    0x766a0abb as libc::c_ulong as uint32_t,
    0x81c2c92e as libc::c_ulong as uint32_t,
    0x92722c85 as libc::c_ulong as uint32_t,
    0xa2bfe8a1 as libc::c_ulong as uint32_t,
    0xa81a664b as libc::c_ulong as uint32_t,
    0xc24b8b70 as libc::c_ulong as uint32_t,
    0xc76c51a3 as libc::c_ulong as uint32_t,
    0xd192e819 as libc::c_ulong as uint32_t,
    0xd6990624 as libc::c_ulong as uint32_t,
    0xf40e3585 as libc::c_ulong as uint32_t,
    0x106aa070 as libc::c_ulong as uint32_t,
    0x19a4c116 as libc::c_ulong as uint32_t,
    0x1e376c08 as libc::c_ulong as uint32_t,
    0x2748774c as libc::c_ulong as uint32_t,
    0x34b0bcb5 as libc::c_ulong as uint32_t,
    0x391c0cb3 as libc::c_ulong as uint32_t,
    0x4ed8aa4a as libc::c_ulong as uint32_t,
    0x5b9cca4f as libc::c_ulong as uint32_t,
    0x682e6ff3 as libc::c_ulong as uint32_t,
    0x748f82ee as libc::c_ulong as uint32_t,
    0x78a5636f as libc::c_ulong as uint32_t,
    0x84c87814 as libc::c_ulong as uint32_t,
    0x8cc70208 as libc::c_ulong as uint32_t,
    0x90befffa as libc::c_ulong as uint32_t,
    0xa4506ceb as libc::c_ulong as uint32_t,
    0xbef9a3f7 as libc::c_ulong as uint32_t,
    0xc67178f2 as libc::c_ulong as uint32_t,
];
pub unsafe extern "C" fn nettle_sha256_compress(
    mut state: *mut uint32_t,
    mut input: *const uint8_t,
) {
    _nettle_sha256_compress_n(state, K.as_ptr(), 1 as libc::c_int as size_t, input);
}
pub unsafe extern "C" fn nettle_sha256_init(mut ctx: *mut sha256_ctx) {
    static mut H0: [uint32_t; 8] = [
        0x6a09e667 as libc::c_ulong as uint32_t,
        0xbb67ae85 as libc::c_ulong as uint32_t,
        0x3c6ef372 as libc::c_ulong as uint32_t,
        0xa54ff53a as libc::c_ulong as uint32_t,
        0x510e527f as libc::c_ulong as uint32_t,
        0x9b05688c as libc::c_ulong as uint32_t,
        0x1f83d9ab as libc::c_ulong as uint32_t,
        0x5be0cd19 as libc::c_ulong as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count = 0 as libc::c_int as uint64_t;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sha256_update(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut blocks: size_t = 0;
    if (*ctx).index > 0 as libc::c_int as libc::c_uint {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 64]>()
            as libc::c_ulong)
            .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
        if length < __md_left as libc::c_ulong {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx)
                .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                as libc::c_uint as libc::c_uint;
            return;
        }
        memcpy(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            data as *const libc::c_void,
            __md_left as libc::c_ulong,
        );
        data = data.offset(__md_left as isize);
        length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
            as size_t as size_t;
        nettle_sha256_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        (*ctx).count = ((*ctx).count).wrapping_add(1);
        (*ctx).count;
    }
    blocks = length >> 6 as libc::c_int;
    data = _nettle_sha256_compress_n(
        ((*ctx).state).as_mut_ptr(),
        K.as_ptr(),
        blocks,
        data,
    );
    (*ctx)
        .count = ((*ctx).count as libc::c_ulong).wrapping_add(blocks) as uint64_t
        as uint64_t;
    length &= 63 as libc::c_int as libc::c_ulong;
    memcpy(
        ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length,
    );
    (*ctx).index = length as libc::c_uint;
}
unsafe extern "C" fn sha256_write_digest(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    if length <= 32 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SHA256_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sha256.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2412: {
        if length <= 32 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SHA256_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sha256.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: libc::c_uint = 0;
    __md_i = (*ctx).index;
    if (__md_i as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
            b"sha256.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2352: {
        if (__md_i as libc::c_ulong)
            < ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
                b"sha256.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh0 as usize] = 0x80 as libc::c_int as uint8_t;
    if __md_i as libc::c_ulong
        > (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
                .wrapping_sub(__md_i as libc::c_ulong),
        );
        nettle_sha256_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        __md_i = 0 as libc::c_int as libc::c_uint;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(__md_i as libc::c_ulong),
    );
    bit_count = (*ctx).count << 9 as libc::c_int
        | ((*ctx).index << 3 as libc::c_int) as libc::c_ulong;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (bit_count >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (bit_count >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (bit_count >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (bit_count >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (bit_count >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (bit_count >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (bit_count >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (bit_count & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    nettle_sha256_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    _nettle_write_be32(length, digest, ((*ctx).state).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_sha256_digest(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    sha256_write_digest(ctx, length, digest);
    nettle_sha256_init(ctx);
}
pub unsafe extern "C" fn nettle_sha224_init(mut ctx: *mut sha256_ctx) {
    static mut H0: [uint32_t; 8] = [
        0xc1059ed8 as libc::c_uint,
        0x367cd507 as libc::c_int as uint32_t,
        0x3070dd17 as libc::c_int as uint32_t,
        0xf70e5939 as libc::c_uint,
        0xffc00b31 as libc::c_uint,
        0x68581511 as libc::c_int as uint32_t,
        0x64f98fa7 as libc::c_int as uint32_t,
        0xbefa4fa4 as libc::c_uint,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count = 0 as libc::c_int as uint64_t;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sha224_digest(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    sha256_write_digest(ctx, length, digest);
    nettle_sha224_init(ctx);
}
