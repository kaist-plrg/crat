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
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes128_decrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_encrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_decrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_decrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
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
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
pub unsafe extern "C" fn nettle_nist_keywrap16(
    mut ctx: *const libc::c_void,
    mut encrypt: Option::<nettle_cipher_func>,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    let mut I: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut B: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut A: nettle_block8 = nettle_block8 { b: [0; 8] };
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0;
    let mut R: *mut uint8_t = ciphertext.offset(8 as libc::c_int as isize);
    if ciphertext_length >= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ciphertext_length >= 16\0" as *const u8 as *const libc::c_char,
            b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 116],
                &[libc::c_char; 116],
            >(
                b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_880: {
        if ciphertext_length >= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ciphertext_length >= 16\0" as *const u8 as *const libc::c_char,
                b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 116],
                    &[libc::c_char; 116],
                >(
                    b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ciphertext_length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(ciphertext_length % 8)\0" as *const u8 as *const libc::c_char,
            b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 116],
                &[libc::c_char; 116],
            >(
                b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_833: {
        if ciphertext_length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0
        {} else {
            __assert_fail(
                b"!(ciphertext_length % 8)\0" as *const u8 as *const libc::c_char,
                b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 116],
                    &[libc::c_char; 116],
                >(
                    b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    n = ciphertext_length
        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    memcpy(
        R as *mut libc::c_void,
        cleartext as *const libc::c_void,
        ciphertext_length.wrapping_sub(8 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        (A.b).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    j = 0 as libc::c_int as size_t;
    while j < 6 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            I.u64_0[0 as libc::c_int as usize] = A.u64_0;
            memcpy(
                (I.b).as_mut_ptr().offset(8 as libc::c_int as isize)
                    as *mut libc::c_void,
                R.offset(i.wrapping_mul(8 as libc::c_int as libc::c_ulong) as isize)
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            encrypt
                .unwrap()(
                ctx,
                16 as libc::c_int as size_t,
                (B.b).as_mut_ptr(),
                (I.b).as_mut_ptr(),
            );
            A
                .u64_0 = B.u64_0[0 as libc::c_int as usize]
                ^ n
                    .wrapping_mul(j)
                    .wrapping_add(i.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    .swap_bytes();
            memcpy(
                R.offset(i.wrapping_mul(8 as libc::c_int as libc::c_ulong) as isize)
                    as *mut libc::c_void,
                (B.b).as_mut_ptr().offset(8 as libc::c_int as isize)
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    memcpy(
        ciphertext as *mut libc::c_void,
        (A.b).as_mut_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
}
pub unsafe extern "C" fn nettle_nist_keyunwrap16(
    mut ctx: *const libc::c_void,
    mut decrypt: Option::<nettle_cipher_func>,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> libc::c_int {
    let mut I: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut B: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut A: nettle_block8 = nettle_block8 { b: [0; 8] };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut R: *mut uint8_t = cleartext;
    if cleartext_length >= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cleartext_length >= 8\0" as *const u8 as *const libc::c_char,
            b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1167: {
        if cleartext_length >= 8 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cleartext_length >= 8\0" as *const u8 as *const libc::c_char,
                b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 117],
                    &[libc::c_char; 117],
                >(
                    b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if cleartext_length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(cleartext_length % 8)\0" as *const u8 as *const libc::c_char,
            b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1124: {
        if cleartext_length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0
        {} else {
            __assert_fail(
                b"!(cleartext_length % 8)\0" as *const u8 as *const libc::c_char,
                b"nist-keywrap.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 117],
                    &[libc::c_char; 117],
                >(
                    b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    n = cleartext_length.wrapping_div(8 as libc::c_int as libc::c_ulong);
    memcpy(
        (A.b).as_mut_ptr() as *mut libc::c_void,
        ciphertext as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        R as *mut libc::c_void,
        ciphertext.offset(8 as libc::c_int as isize) as *const libc::c_void,
        cleartext_length,
    );
    j = 5 as libc::c_int;
    while j >= 0 as libc::c_int {
        i = n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while i >= 0 as libc::c_int {
            I
                .u64_0[0 as libc::c_int
                as usize] = A.u64_0
                ^ n
                    .wrapping_mul(j as libc::c_ulong)
                    .wrapping_add((i + 1 as libc::c_int) as libc::c_ulong)
                    .swap_bytes();
            memcpy(
                (I.b).as_mut_ptr().offset(8 as libc::c_int as isize)
                    as *mut libc::c_void,
                R.offset((i * 8 as libc::c_int) as isize) as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            decrypt
                .unwrap()(
                ctx,
                16 as libc::c_int as size_t,
                (B.b).as_mut_ptr(),
                (I.b).as_mut_ptr(),
            );
            A.u64_0 = B.u64_0[0 as libc::c_int as usize];
            memcpy(
                R.offset((i * 8 as libc::c_int) as isize) as *mut libc::c_void,
                (B.b).as_mut_ptr().offset(8 as libc::c_int as isize)
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            i -= 1;
            i;
        }
        j -= 1;
        j;
    }
    return nettle_memeql_sec(
        (A.b).as_mut_ptr() as *const libc::c_void,
        iv as *const libc::c_void,
        8 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn nettle_aes128_keywrap(
    mut ctx: *mut aes128_ctx,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    nettle_nist_keywrap16(
        ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes128_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes128_encrypt
                    as unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        ciphertext_length,
        ciphertext,
        cleartext,
    );
}
pub unsafe extern "C" fn nettle_aes192_keywrap(
    mut ctx: *mut aes192_ctx,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    nettle_nist_keywrap16(
        ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes192_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes192_encrypt
                    as unsafe extern "C" fn(
                        *const aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        ciphertext_length,
        ciphertext,
        cleartext,
    );
}
pub unsafe extern "C" fn nettle_aes256_keywrap(
    mut ctx: *mut aes256_ctx,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    nettle_nist_keywrap16(
        ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        ciphertext_length,
        ciphertext,
        cleartext,
    );
}
pub unsafe extern "C" fn nettle_aes128_keyunwrap(
    mut ctx: *mut aes128_ctx,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> libc::c_int {
    return nettle_nist_keyunwrap16(
        ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes128_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes128_decrypt
                    as unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        cleartext_length,
        cleartext,
        ciphertext,
    );
}
pub unsafe extern "C" fn nettle_aes192_keyunwrap(
    mut ctx: *mut aes192_ctx,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> libc::c_int {
    return nettle_nist_keyunwrap16(
        ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes192_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes192_decrypt
                    as unsafe extern "C" fn(
                        *const aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        cleartext_length,
        cleartext,
        ciphertext,
    );
}
pub unsafe extern "C" fn nettle_aes256_keyunwrap(
    mut ctx: *mut aes256_ctx,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> libc::c_int {
    return nettle_nist_keyunwrap16(
        ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_decrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        cleartext_length,
        cleartext,
        ciphertext,
    );
}
