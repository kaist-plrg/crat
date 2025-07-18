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
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub unsafe extern "C" fn nettle_cbc_encrypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(block_size) == 0 {} else {
        __assert_fail(
            b"!(length % block_size)\0" as *const u8 as *const libc::c_char,
            b"cbc.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"void nettle_cbc_encrypt(const void *, nettle_cipher_func *, size_t, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1750: {
        if length.wrapping_rem(block_size) == 0 {} else {
            __assert_fail(
                b"!(length % block_size)\0" as *const u8 as *const libc::c_char,
                b"cbc.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"void nettle_cbc_encrypt(const void *, nettle_cipher_func *, size_t, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        nettle_memxor(iv as *mut libc::c_void, src as *const libc::c_void, block_size);
        f.unwrap()(ctx, block_size, dst, iv);
        memcpy(iv as *mut libc::c_void, dst as *const libc::c_void, block_size);
        length = (length as libc::c_ulong).wrapping_sub(block_size) as size_t as size_t;
        src = src.offset(block_size as isize);
        dst = dst.offset(block_size as isize);
    }
}
pub unsafe extern "C" fn nettle_cbc_decrypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(block_size) == 0 {} else {
        __assert_fail(
            b"!(length % block_size)\0" as *const u8 as *const libc::c_char,
            b"cbc.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"void nettle_cbc_decrypt(const void *, nettle_cipher_func *, size_t, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2138: {
        if length.wrapping_rem(block_size) == 0 {} else {
            __assert_fail(
                b"!(length % block_size)\0" as *const u8 as *const libc::c_char,
                b"cbc.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"void nettle_cbc_decrypt(const void *, nettle_cipher_func *, size_t, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 0 {
        return;
    }
    if src != dst as *const uint8_t {
        f.unwrap()(ctx, length, dst, src);
        nettle_memxor(dst as *mut libc::c_void, iv as *const libc::c_void, block_size);
        nettle_memxor(
            dst.offset(block_size as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            length.wrapping_sub(block_size),
        );
        memcpy(
            iv as *mut libc::c_void,
            src.offset(length as isize).offset(-(block_size as isize))
                as *const libc::c_void,
            block_size,
        );
    } else {
        let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
        let mut initial_iv: *mut uint8_t = 0 as *mut uint8_t;
        let mut buffer_size: size_t = 0;
        if length <= 512 as libc::c_int as libc::c_ulong {
            buffer_size = length;
        } else {
            buffer_size = (512 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    (512 as libc::c_int as libc::c_ulong).wrapping_rem(block_size),
                );
        }
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(buffer_size)
                as usize,
        );
        buffer = fresh0.leak().as_mut_ptr() as *mut uint8_t;
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(block_size)
                as usize,
        );
        initial_iv = fresh1.leak().as_mut_ptr() as *mut uint8_t;
        while length > buffer_size {
            f.unwrap()(ctx, buffer_size, buffer, dst);
            memcpy(
                initial_iv as *mut libc::c_void,
                iv as *const libc::c_void,
                block_size,
            );
            memcpy(
                iv as *mut libc::c_void,
                dst.offset(buffer_size as isize).offset(-(block_size as isize))
                    as *const libc::c_void,
                block_size,
            );
            nettle_memxor3(
                dst.offset(block_size as isize) as *mut libc::c_void,
                buffer.offset(block_size as isize) as *const libc::c_void,
                dst as *const libc::c_void,
                buffer_size.wrapping_sub(block_size),
            );
            nettle_memxor3(
                dst as *mut libc::c_void,
                buffer as *const libc::c_void,
                initial_iv as *const libc::c_void,
                block_size,
            );
            length = (length as libc::c_ulong).wrapping_sub(buffer_size) as size_t
                as size_t;
            dst = dst.offset(buffer_size as isize);
        }
        f.unwrap()(ctx, length, buffer, dst);
        memcpy(initial_iv as *mut libc::c_void, iv as *const libc::c_void, block_size);
        memcpy(
            iv as *mut libc::c_void,
            dst.offset(length as isize).offset(-(block_size as isize))
                as *const libc::c_void,
            block_size,
        );
        nettle_memxor3(
            dst.offset(block_size as isize) as *mut libc::c_void,
            buffer.offset(block_size as isize) as *const libc::c_void,
            dst as *const libc::c_void,
            length.wrapping_sub(block_size),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            buffer as *const libc::c_void,
            initial_iv as *const libc::c_void,
            block_size,
        );
    };
}
