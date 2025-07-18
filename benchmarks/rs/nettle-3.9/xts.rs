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
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[inline]
unsafe extern "C" fn block16_mulx_le(
    mut dst: *mut nettle_block16,
    mut src: *const nettle_block16,
) {
    let mut carry: uint64_t = (*src).u64_0[1 as libc::c_int as usize]
        >> 63 as libc::c_int;
    (*dst)
        .u64_0[1 as libc::c_int
        as usize] = (*src).u64_0[1 as libc::c_int as usize] << 1 as libc::c_int
        | (*src).u64_0[0 as libc::c_int as usize] >> 63 as libc::c_int;
    (*dst)
        .u64_0[0 as libc::c_int
        as usize] = (*src).u64_0[0 as libc::c_int as usize] << 1 as libc::c_int
        ^ 0x87 as libc::c_ulong & carry.wrapping_neg();
}
unsafe extern "C" fn check_length(mut length: size_t, mut dst: *mut uint8_t) {
    if length >= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length >= XTS_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
            b"xts.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void check_length(size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2078: {
        if length >= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length >= XTS_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"xts.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void check_length(size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if length < 16 as libc::c_int as libc::c_ulong {
        memset(dst as *mut libc::c_void, '\0' as i32, length);
    }
}
pub unsafe extern "C" fn nettle_xts_encrypt_message(
    mut enc_ctx: *const libc::c_void,
    mut twk_ctx: *const libc::c_void,
    mut encf: Option::<nettle_cipher_func>,
    mut tweak: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut T: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut P: nettle_block16 = nettle_block16 { b: [0; 16] };
    check_length(length, dst);
    encf.unwrap()(twk_ctx, 16 as libc::c_int as size_t, (T.b).as_mut_ptr(), tweak);
    while length >= (2 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
        || length == 16 as libc::c_int as libc::c_ulong
    {
        nettle_memxor3(
            (P.b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        encf.unwrap()(enc_ctx, 16 as libc::c_int as size_t, dst, (P.b).as_mut_ptr());
        nettle_memxor(
            dst as *mut libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        if length > 16 as libc::c_int as libc::c_ulong {
            block16_mulx_le(&mut T, &mut T);
        }
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
        src = src.offset(16 as libc::c_int as isize);
        dst = dst.offset(16 as libc::c_int as isize);
    }
    if length != 0 {
        let mut S: nettle_block16 = nettle_block16 { b: [0; 16] };
        nettle_memxor3(
            (P.b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        encf
            .unwrap()(
            enc_ctx,
            16 as libc::c_int as size_t,
            (S.b).as_mut_ptr(),
            (P.b).as_mut_ptr(),
        );
        nettle_memxor(
            (S.b).as_mut_ptr() as *mut libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        block16_mulx_le(&mut T, &mut T);
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
        src = src.offset(16 as libc::c_int as isize);
        nettle_memxor3(
            (P.b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            length,
        );
        nettle_memxor3(
            (P.b).as_mut_ptr().offset(length as isize) as *mut libc::c_void,
            (S.b).as_mut_ptr().offset(length as isize) as *const libc::c_void,
            (T.b).as_mut_ptr().offset(length as isize) as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong).wrapping_sub(length),
        );
        encf.unwrap()(enc_ctx, 16 as libc::c_int as size_t, dst, (P.b).as_mut_ptr());
        nettle_memxor(
            dst as *mut libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        dst = dst.offset(16 as libc::c_int as isize);
        memcpy(
            dst as *mut libc::c_void,
            (S.b).as_mut_ptr() as *const libc::c_void,
            length,
        );
    }
}
pub unsafe extern "C" fn nettle_xts_decrypt_message(
    mut dec_ctx: *const libc::c_void,
    mut twk_ctx: *const libc::c_void,
    mut decf: Option::<nettle_cipher_func>,
    mut encf: Option::<nettle_cipher_func>,
    mut tweak: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut T: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut C: nettle_block16 = nettle_block16 { b: [0; 16] };
    check_length(length, dst);
    encf.unwrap()(twk_ctx, 16 as libc::c_int as size_t, (T.b).as_mut_ptr(), tweak);
    while length >= (2 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
        || length == 16 as libc::c_int as libc::c_ulong
    {
        nettle_memxor3(
            (C.b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        decf.unwrap()(dec_ctx, 16 as libc::c_int as size_t, dst, (C.b).as_mut_ptr());
        nettle_memxor(
            dst as *mut libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        if length > 16 as libc::c_int as libc::c_ulong {
            block16_mulx_le(&mut T, &mut T);
        }
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
        src = src.offset(16 as libc::c_int as isize);
        dst = dst.offset(16 as libc::c_int as isize);
    }
    if length != 0 {
        let mut T1: nettle_block16 = nettle_block16 { b: [0; 16] };
        let mut S: nettle_block16 = nettle_block16 { b: [0; 16] };
        block16_mulx_le(&mut T1, &mut T);
        nettle_memxor3(
            (C.b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            (T1.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        decf
            .unwrap()(
            dec_ctx,
            16 as libc::c_int as size_t,
            (S.b).as_mut_ptr(),
            (C.b).as_mut_ptr(),
        );
        nettle_memxor(
            (S.b).as_mut_ptr() as *mut libc::c_void,
            (T1.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
        src = src.offset(16 as libc::c_int as isize);
        nettle_memxor3(
            (C.b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            length,
        );
        nettle_memxor3(
            (C.b).as_mut_ptr().offset(length as isize) as *mut libc::c_void,
            (S.b).as_mut_ptr().offset(length as isize) as *const libc::c_void,
            (T.b).as_mut_ptr().offset(length as isize) as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong).wrapping_sub(length),
        );
        decf.unwrap()(dec_ctx, 16 as libc::c_int as size_t, dst, (C.b).as_mut_ptr());
        nettle_memxor(
            dst as *mut libc::c_void,
            (T.b).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        dst = dst.offset(16 as libc::c_int as isize);
        memcpy(
            dst as *mut libc::c_void,
            (S.b).as_mut_ptr() as *const libc::c_void,
            length,
        );
    }
}
