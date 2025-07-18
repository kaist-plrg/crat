use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_sha512_compress(
        state: *mut uint64_t,
        data: *const uint8_t,
        k: *const uint64_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 128],
}
static mut K: [uint64_t; 80] = [
    0x428a2f98d728ae22 as libc::c_ulonglong as uint64_t,
    0x7137449123ef65cd as libc::c_ulonglong as uint64_t,
    0xb5c0fbcfec4d3b2f as libc::c_ulonglong as uint64_t,
    0xe9b5dba58189dbbc as libc::c_ulonglong as uint64_t,
    0x3956c25bf348b538 as libc::c_ulonglong as uint64_t,
    0x59f111f1b605d019 as libc::c_ulonglong as uint64_t,
    0x923f82a4af194f9b as libc::c_ulonglong as uint64_t,
    0xab1c5ed5da6d8118 as libc::c_ulonglong as uint64_t,
    0xd807aa98a3030242 as libc::c_ulonglong as uint64_t,
    0x12835b0145706fbe as libc::c_ulonglong as uint64_t,
    0x243185be4ee4b28c as libc::c_ulonglong as uint64_t,
    0x550c7dc3d5ffb4e2 as libc::c_ulonglong as uint64_t,
    0x72be5d74f27b896f as libc::c_ulonglong as uint64_t,
    0x80deb1fe3b1696b1 as libc::c_ulonglong as uint64_t,
    0x9bdc06a725c71235 as libc::c_ulonglong as uint64_t,
    0xc19bf174cf692694 as libc::c_ulonglong as uint64_t,
    0xe49b69c19ef14ad2 as libc::c_ulonglong as uint64_t,
    0xefbe4786384f25e3 as libc::c_ulonglong as uint64_t,
    0xfc19dc68b8cd5b5 as libc::c_ulonglong as uint64_t,
    0x240ca1cc77ac9c65 as libc::c_ulonglong as uint64_t,
    0x2de92c6f592b0275 as libc::c_ulonglong as uint64_t,
    0x4a7484aa6ea6e483 as libc::c_ulonglong as uint64_t,
    0x5cb0a9dcbd41fbd4 as libc::c_ulonglong as uint64_t,
    0x76f988da831153b5 as libc::c_ulonglong as uint64_t,
    0x983e5152ee66dfab as libc::c_ulonglong as uint64_t,
    0xa831c66d2db43210 as libc::c_ulonglong as uint64_t,
    0xb00327c898fb213f as libc::c_ulonglong as uint64_t,
    0xbf597fc7beef0ee4 as libc::c_ulonglong as uint64_t,
    0xc6e00bf33da88fc2 as libc::c_ulonglong as uint64_t,
    0xd5a79147930aa725 as libc::c_ulonglong as uint64_t,
    0x6ca6351e003826f as libc::c_ulonglong as uint64_t,
    0x142929670a0e6e70 as libc::c_ulonglong as uint64_t,
    0x27b70a8546d22ffc as libc::c_ulonglong as uint64_t,
    0x2e1b21385c26c926 as libc::c_ulonglong as uint64_t,
    0x4d2c6dfc5ac42aed as libc::c_ulonglong as uint64_t,
    0x53380d139d95b3df as libc::c_ulonglong as uint64_t,
    0x650a73548baf63de as libc::c_ulonglong as uint64_t,
    0x766a0abb3c77b2a8 as libc::c_ulonglong as uint64_t,
    0x81c2c92e47edaee6 as libc::c_ulonglong as uint64_t,
    0x92722c851482353b as libc::c_ulonglong as uint64_t,
    0xa2bfe8a14cf10364 as libc::c_ulonglong as uint64_t,
    0xa81a664bbc423001 as libc::c_ulonglong as uint64_t,
    0xc24b8b70d0f89791 as libc::c_ulonglong as uint64_t,
    0xc76c51a30654be30 as libc::c_ulonglong as uint64_t,
    0xd192e819d6ef5218 as libc::c_ulonglong as uint64_t,
    0xd69906245565a910 as libc::c_ulonglong as uint64_t,
    0xf40e35855771202a as libc::c_ulonglong as uint64_t,
    0x106aa07032bbd1b8 as libc::c_ulonglong as uint64_t,
    0x19a4c116b8d2d0c8 as libc::c_ulonglong as uint64_t,
    0x1e376c085141ab53 as libc::c_ulonglong as uint64_t,
    0x2748774cdf8eeb99 as libc::c_ulonglong as uint64_t,
    0x34b0bcb5e19b48a8 as libc::c_ulonglong as uint64_t,
    0x391c0cb3c5c95a63 as libc::c_ulonglong as uint64_t,
    0x4ed8aa4ae3418acb as libc::c_ulonglong as uint64_t,
    0x5b9cca4f7763e373 as libc::c_ulonglong as uint64_t,
    0x682e6ff3d6b2b8a3 as libc::c_ulonglong as uint64_t,
    0x748f82ee5defb2fc as libc::c_ulonglong as uint64_t,
    0x78a5636f43172f60 as libc::c_ulonglong as uint64_t,
    0x84c87814a1f0ab72 as libc::c_ulonglong as uint64_t,
    0x8cc702081a6439ec as libc::c_ulonglong as uint64_t,
    0x90befffa23631e28 as libc::c_ulonglong as uint64_t,
    0xa4506cebde82bde9 as libc::c_ulonglong as uint64_t,
    0xbef9a3f7b2c67915 as libc::c_ulonglong as uint64_t,
    0xc67178f2e372532b as libc::c_ulonglong as uint64_t,
    0xca273eceea26619c as libc::c_ulonglong as uint64_t,
    0xd186b8c721c0c207 as libc::c_ulonglong as uint64_t,
    0xeada7dd6cde0eb1e as libc::c_ulonglong as uint64_t,
    0xf57d4f7fee6ed178 as libc::c_ulonglong as uint64_t,
    0x6f067aa72176fba as libc::c_ulonglong as uint64_t,
    0xa637dc5a2c898a6 as libc::c_ulonglong as uint64_t,
    0x113f9804bef90dae as libc::c_ulonglong as uint64_t,
    0x1b710b35131c471b as libc::c_ulonglong as uint64_t,
    0x28db77f523047d84 as libc::c_ulonglong as uint64_t,
    0x32caab7b40c72493 as libc::c_ulonglong as uint64_t,
    0x3c9ebe0a15c9bebc as libc::c_ulonglong as uint64_t,
    0x431d67c49c100d4c as libc::c_ulonglong as uint64_t,
    0x4cc5d4becb3e42b6 as libc::c_ulonglong as uint64_t,
    0x597f299cfc657e2a as libc::c_ulonglong as uint64_t,
    0x5fcb6fab3ad6faec as libc::c_ulonglong as uint64_t,
    0x6c44198c4a475817 as libc::c_ulonglong as uint64_t,
];
pub unsafe extern "C" fn nettle_sha512_init(mut ctx: *mut sha512_ctx) {
    static mut H0: [uint64_t; 8] = [
        0x6a09e667f3bcc908 as libc::c_ulonglong as uint64_t,
        0xbb67ae8584caa73b as libc::c_ulonglong as uint64_t,
        0x3c6ef372fe94f82b as libc::c_ulonglong as uint64_t,
        0xa54ff53a5f1d36f1 as libc::c_ulonglong as uint64_t,
        0x510e527fade682d1 as libc::c_ulonglong as uint64_t,
        0x9b05688c2b3e6c1f as libc::c_ulonglong as uint64_t,
        0x1f83d9abfb41bd6b as libc::c_ulonglong as uint64_t,
        0x5be0cd19137e2179 as libc::c_ulonglong as uint64_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count_high = 0 as libc::c_int as uint64_t;
    (*ctx).count_low = (*ctx).count_high;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sha512_update(
    mut ctx: *mut sha512_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_16: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 128]>()
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
            current_block_16 = 13056961889198038528;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            nettle_sha512_compress(
                ((*ctx).state).as_mut_ptr(),
                ((*ctx).block).as_mut_ptr(),
            );
            (*ctx).count_low = ((*ctx).count_low).wrapping_add(1);
            (*ctx)
                .count_high = ((*ctx).count_high as libc::c_ulong)
                .wrapping_add(((*ctx).count_low == 0) as libc::c_int as libc::c_ulong)
                as uint64_t as uint64_t;
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_16 = 5720623009719927633;
        }
    } else {
        current_block_16 = 5720623009719927633;
    }
    match current_block_16 {
        5720623009719927633 => {
            while length >= ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong {
                nettle_sha512_compress(((*ctx).state).as_mut_ptr(), data);
                (*ctx).count_low = ((*ctx).count_low).wrapping_add(1);
                (*ctx)
                    .count_high = ((*ctx).count_high as libc::c_ulong)
                    .wrapping_add(
                        ((*ctx).count_low == 0) as libc::c_int as libc::c_ulong,
                    ) as uint64_t as uint64_t;
                data = data
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            memcpy(
                ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx).index = length as libc::c_uint;
        }
        _ => {}
    };
}
unsafe extern "C" fn sha512_write_digest(
    mut ctx: *mut sha512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut high: uint64_t = 0;
    let mut low: uint64_t = 0;
    let mut i: libc::c_uint = 0;
    let mut words: libc::c_uint = 0;
    let mut leftover: libc::c_uint = 0;
    if length <= 64 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SHA512_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sha512.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void sha512_write_digest(struct sha512_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2990: {
        if length <= 64 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SHA512_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sha512.c\0" as *const u8 as *const libc::c_char,
                165 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void sha512_write_digest(struct sha512_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: libc::c_uint = 0;
    __md_i = (*ctx).index;
    if (__md_i as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
            b"sha512.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void sha512_write_digest(struct sha512_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2930: {
        if (__md_i as libc::c_ulong)
            < ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
                b"sha512.c\0" as *const u8 as *const libc::c_char,
                167 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void sha512_write_digest(struct sha512_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh0 as usize] = 0x80 as libc::c_int as uint8_t;
    if __md_i as libc::c_ulong
        > (::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong)
                .wrapping_sub(__md_i as libc::c_ulong),
        );
        nettle_sha512_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        __md_i = 0 as libc::c_int as libc::c_uint;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong)
            .wrapping_sub(__md_i as libc::c_ulong),
    );
    high = (*ctx).count_high << 10 as libc::c_int
        | (*ctx).count_low >> 54 as libc::c_int;
    low = (*ctx).count_low << 10 as libc::c_int
        | ((*ctx).index << 3 as libc::c_int) as libc::c_ulong;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (high >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (high >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (high >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (high >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (high >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (high >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (high >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 16 as libc::c_int) as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (high & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (low >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (low >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (low >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (low >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (low >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (low >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (low >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((128 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (low & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    nettle_sha512_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    words = length.wrapping_div(8 as libc::c_int as libc::c_ulong) as libc::c_uint;
    leftover = length.wrapping_rem(8 as libc::c_int as libc::c_ulong) as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < words {
        *digest
            .offset(
                0 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 56 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                1 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 48 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                2 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 40 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                3 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                4 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                5 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                6 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        *digest
            .offset(
                7 as libc::c_int as isize,
            ) = ((*ctx).state[i as usize] & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        i = i.wrapping_add(1);
        i;
        digest = digest.offset(8 as libc::c_int as isize);
    }
    if leftover != 0 {
        let mut word: uint64_t = (*ctx).state[i as usize]
            >> (8 as libc::c_int as libc::c_uint)
                .wrapping_mul((8 as libc::c_int as libc::c_uint).wrapping_sub(leftover));
        loop {
            leftover = leftover.wrapping_sub(1);
            *digest
                .offset(
                    leftover as isize,
                ) = (word & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            word >>= 8 as libc::c_int;
            if !(leftover != 0) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn nettle_sha512_digest(
    mut ctx: *mut sha512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length <= 64 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SHA512_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sha512.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_sha512_digest(struct sha512_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3029: {
        if length <= 64 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SHA512_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sha512.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_sha512_digest(struct sha512_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sha512_write_digest(ctx, length, digest);
    nettle_sha512_init(ctx);
}
pub unsafe extern "C" fn nettle_sha384_init(mut ctx: *mut sha512_ctx) {
    static mut H0: [uint64_t; 8] = [
        0xcbbb9d5dc1059ed8 as libc::c_ulonglong as uint64_t,
        0x629a292a367cd507 as libc::c_ulonglong as uint64_t,
        0x9159015a3070dd17 as libc::c_ulonglong as uint64_t,
        0x152fecd8f70e5939 as libc::c_ulonglong as uint64_t,
        0x67332667ffc00b31 as libc::c_ulonglong as uint64_t,
        0x8eb44a8768581511 as libc::c_ulonglong as uint64_t,
        0xdb0c2e0d64f98fa7 as libc::c_ulonglong as uint64_t,
        0x47b5481dbefa4fa4 as libc::c_ulonglong as uint64_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count_high = 0 as libc::c_int as uint64_t;
    (*ctx).count_low = (*ctx).count_high;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sha384_digest(
    mut ctx: *mut sha512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length <= 48 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SHA384_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sha512.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_sha384_digest(struct sha512_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3147: {
        if length <= 48 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SHA384_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sha512.c\0" as *const u8 as *const libc::c_char,
                244 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_sha384_digest(struct sha512_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sha512_write_digest(ctx, length, digest);
    nettle_sha384_init(ctx);
}
pub unsafe extern "C" fn nettle_sha512_224_init(mut ctx: *mut sha512_ctx) {
    static mut H0: [uint64_t; 8] = [
        0x8c3d37c819544da2 as libc::c_ulonglong as uint64_t,
        0x73e1996689dcd4d6 as libc::c_ulonglong as uint64_t,
        0x1dfab7ae32ff9c82 as libc::c_ulonglong as uint64_t,
        0x679dd514582f9fcf as libc::c_ulonglong as uint64_t,
        0xf6d2b697bd44da8 as libc::c_ulonglong as uint64_t,
        0x77e36f7304c48942 as libc::c_ulonglong as uint64_t,
        0x3f9d85a86a1d36c8 as libc::c_ulonglong as uint64_t,
        0x1112e6ad91d692a1 as libc::c_ulonglong as uint64_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count_high = 0 as libc::c_int as uint64_t;
    (*ctx).count_low = (*ctx).count_high;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sha512_224_digest(
    mut ctx: *mut sha512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length <= 28 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SHA224_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sha512.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"void nettle_sha512_224_digest(struct sha512_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3264: {
        if length <= 28 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SHA224_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sha512.c\0" as *const u8 as *const libc::c_char,
                277 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"void nettle_sha512_224_digest(struct sha512_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sha512_write_digest(ctx, length, digest);
    nettle_sha512_224_init(ctx);
}
pub unsafe extern "C" fn nettle_sha512_256_init(mut ctx: *mut sha512_ctx) {
    static mut H0: [uint64_t; 8] = [
        0x22312194fc2bf72c as libc::c_ulonglong as uint64_t,
        0x9f555fa3c84c64c2 as libc::c_ulonglong as uint64_t,
        0x2393b86b6f53b151 as libc::c_ulonglong as uint64_t,
        0x963877195940eabd as libc::c_ulonglong as uint64_t,
        0x96283ee2a88effe3 as libc::c_ulonglong as uint64_t,
        0xbe5e1e2553863992 as libc::c_ulonglong as uint64_t,
        0x2b0199fc2c85b8aa as libc::c_ulonglong as uint64_t,
        0xeb72ddc81c52ca2 as libc::c_ulonglong as uint64_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count_high = 0 as libc::c_int as uint64_t;
    (*ctx).count_low = (*ctx).count_high;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sha512_256_digest(
    mut ctx: *mut sha512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length <= 32 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SHA256_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sha512.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"void nettle_sha512_256_digest(struct sha512_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3382: {
        if length <= 32 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SHA256_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sha512.c\0" as *const u8 as *const libc::c_char,
                310 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"void nettle_sha512_256_digest(struct sha512_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sha512_write_digest(ctx, length, digest);
    nettle_sha512_256_init(ctx);
}
pub unsafe extern "C" fn nettle_sha512_compress(
    mut state: *mut uint64_t,
    mut input: *const uint8_t,
) {
    _nettle_sha512_compress(state, input, K.as_ptr());
}
