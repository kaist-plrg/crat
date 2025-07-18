use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _nettle_camellia_crypt(
        nkeys: libc::c_uint,
        keys: *const uint64_t,
        T: *const camellia_table,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    static _nettle_camellia_table: camellia_table;
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
pub struct camellia256_ctx {
    pub keys: [uint64_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia_table {
    pub sp1110: [uint32_t; 256],
    pub sp0222: [uint32_t; 256],
    pub sp3033: [uint32_t; 256],
    pub sp4404: [uint32_t; 256],
}
pub unsafe extern "C" fn nettle_camellia256_crypt(
    mut ctx: *const camellia256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(16 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(length % CAMELLIA_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
            b"camellia256-crypt.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void nettle_camellia256_crypt(const struct camellia256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_322: {
        if length.wrapping_rem(16 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!(length % CAMELLIA_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
                b"camellia256-crypt.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void nettle_camellia256_crypt(const struct camellia256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_camellia_crypt(
        32 as libc::c_int as libc::c_uint,
        ((*ctx).keys).as_ptr(),
        &_nettle_camellia_table,
        length,
        dst,
        src,
    );
}
