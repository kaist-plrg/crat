use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static _nettle_aes_decrypt_table: aes_table;
    fn _nettle_aes_decrypt(
        rounds: libc::c_uint,
        keys: *const uint32_t,
        T: *const aes_table,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_table {
    pub sbox: [uint8_t; 256],
    pub table: [[uint32_t; 256]; 4],
}
pub unsafe extern "C" fn _nettle_aes192_decrypt_c(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(16 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(length % AES_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
            b"aes192-decrypt.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"void _nettle_aes192_decrypt_c(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_423: {
        if length.wrapping_rem(16 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!(length % AES_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
                b"aes192-decrypt.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"void _nettle_aes192_decrypt_c(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_aes_decrypt(
        12 as libc::c_int as libc::c_uint,
        ((*ctx).keys).as_ptr(),
        &_nettle_aes_decrypt_table,
        length,
        dst,
        src,
    );
}
