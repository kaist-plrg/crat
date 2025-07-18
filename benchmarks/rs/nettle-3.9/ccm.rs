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
    fn nettle_ctr_crypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccm_ctx {
    pub ctr: nettle_block16,
    pub tag: nettle_block16,
    pub blength: libc::c_uint,
}
unsafe extern "C" fn ccm_pad(
    mut ctx: *mut ccm_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
) {
    if (*ctx).blength != 0 {
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            ((*ctx).tag.b).as_mut_ptr(),
            ((*ctx).tag.b).as_mut_ptr(),
        );
    }
    (*ctx).blength = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ccm_build_iv(
    mut iv: *mut uint8_t,
    mut noncelen: size_t,
    mut nonce: *const uint8_t,
    mut flags: uint8_t,
    mut count: size_t,
) {
    let mut i: libc::c_uint = 0;
    if noncelen >= 7 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"noncelen >= CCM_MIN_NONCE_SIZE\0" as *const u8 as *const libc::c_char,
            b"ccm.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void ccm_build_iv(uint8_t *, size_t, const uint8_t *, uint8_t, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2229: {
        if noncelen >= 7 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"noncelen >= CCM_MIN_NONCE_SIZE\0" as *const u8 as *const libc::c_char,
                b"ccm.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"void ccm_build_iv(uint8_t *, size_t, const uint8_t *, uint8_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if noncelen <= 14 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"noncelen <= CCM_MAX_NONCE_SIZE\0" as *const u8 as *const libc::c_char,
            b"ccm.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void ccm_build_iv(uint8_t *, size_t, const uint8_t *, uint8_t, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2190: {
        if noncelen <= 14 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"noncelen <= CCM_MAX_NONCE_SIZE\0" as *const u8 as *const libc::c_char,
                b"ccm.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"void ccm_build_iv(uint8_t *, size_t, const uint8_t *, uint8_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *iv
        .offset(
            0 as libc::c_int as isize,
        ) = (flags as libc::c_ulong
        | ((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_sub(noncelen)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & 0x7 as libc::c_int as libc::c_ulong) as uint8_t;
    memcpy(
        &mut *iv.offset(1 as libc::c_int as isize) as *mut uint8_t as *mut libc::c_void,
        nonce as *const libc::c_void,
        noncelen,
    );
    i = (16 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    while i as libc::c_ulong
        >= (1 as libc::c_int as libc::c_ulong).wrapping_add(noncelen)
    {
        *iv
            .offset(
                i as isize,
            ) = (count & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        count >>= 8 as libc::c_int;
        i = i.wrapping_sub(1);
        i;
    }
    if count == 0 {} else {
        __assert_fail(
            b"!count\0" as *const u8 as *const libc::c_char,
            b"ccm.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void ccm_build_iv(uint8_t *, size_t, const uint8_t *, uint8_t, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2066: {
        if count == 0 {} else {
            __assert_fail(
                b"!count\0" as *const u8 as *const libc::c_char,
                b"ccm.c\0" as *const u8 as *const libc::c_char,
                116 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"void ccm_build_iv(uint8_t *, size_t, const uint8_t *, uint8_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn nettle_ccm_set_nonce(
    mut ctx: *mut ccm_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut nonce: *const uint8_t,
    mut authlen: size_t,
    mut msglen: size_t,
    mut taglen: size_t,
) {
    (*ctx).blength = 0 as libc::c_int as libc::c_uint;
    ccm_build_iv(
        ((*ctx).tag.b).as_mut_ptr(),
        length,
        nonce,
        (taglen.wrapping_sub(2 as libc::c_int as libc::c_ulong) << 2 as libc::c_int
            & 0x38 as libc::c_int as libc::c_ulong) as uint8_t,
        msglen,
    );
    ccm_build_iv(
        ((*ctx).ctr.b).as_mut_ptr(),
        length,
        nonce,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as size_t,
    );
    if authlen == 0 {
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            ((*ctx).tag.b).as_mut_ptr(),
            ((*ctx).tag.b).as_mut_ptr(),
        );
        return;
    }
    (*ctx)
        .tag
        .b[0 as libc::c_int
        as usize] = ((*ctx).tag.b[0 as libc::c_int as usize] as libc::c_int
        | 0x40 as libc::c_int) as uint8_t;
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        ((*ctx).tag.b).as_mut_ptr(),
        ((*ctx).tag.b).as_mut_ptr(),
    );
    if authlen as libc::c_ulonglong >= (0x1 as libc::c_ulonglong) << 32 as libc::c_int {
        let fresh0 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh0
            as usize] = ((*ctx).tag.b[fresh0 as usize] as libc::c_int
            ^ 0xff as libc::c_int) as uint8_t;
        let fresh1 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh1
            as usize] = ((*ctx).tag.b[fresh1 as usize] as libc::c_int
            ^ 0xff as libc::c_int) as uint8_t;
        let fresh2 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh2
            as usize] = ((*ctx).tag.b[fresh2 as usize] as libc::c_ulong
            ^ authlen >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        let fresh3 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh3
            as usize] = ((*ctx).tag.b[fresh3 as usize] as libc::c_ulong
            ^ authlen >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        let fresh4 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh4
            as usize] = ((*ctx).tag.b[fresh4 as usize] as libc::c_ulong
            ^ authlen >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        let fresh5 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh5
            as usize] = ((*ctx).tag.b[fresh5 as usize] as libc::c_ulong
            ^ authlen >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        let fresh6 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh6
            as usize] = ((*ctx).tag.b[fresh6 as usize] as libc::c_ulong
            ^ authlen >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        let fresh7 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh7
            as usize] = ((*ctx).tag.b[fresh7 as usize] as libc::c_ulong
            ^ authlen >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
    } else if authlen as libc::c_ulonglong
        >= ((0x1 as libc::c_ulonglong) << 16 as libc::c_int)
            .wrapping_sub((0x1 as libc::c_ulonglong) << 8 as libc::c_int)
    {
        let fresh8 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh8
            as usize] = ((*ctx).tag.b[fresh8 as usize] as libc::c_int
            ^ 0xff as libc::c_int) as uint8_t;
        let fresh9 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh9
            as usize] = ((*ctx).tag.b[fresh9 as usize] as libc::c_int
            ^ 0xfe as libc::c_int) as uint8_t;
        let fresh10 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh10
            as usize] = ((*ctx).tag.b[fresh10 as usize] as libc::c_ulong
            ^ authlen >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        let fresh11 = (*ctx).blength;
        (*ctx).blength = ((*ctx).blength).wrapping_add(1);
        (*ctx)
            .tag
            .b[fresh11
            as usize] = ((*ctx).tag.b[fresh11 as usize] as libc::c_ulong
            ^ authlen >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
    }
    let fresh12 = (*ctx).blength;
    (*ctx).blength = ((*ctx).blength).wrapping_add(1);
    (*ctx)
        .tag
        .b[fresh12
        as usize] = ((*ctx).tag.b[fresh12 as usize] as libc::c_ulong
        ^ authlen >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    let fresh13 = (*ctx).blength;
    (*ctx).blength = ((*ctx).blength).wrapping_add(1);
    (*ctx)
        .tag
        .b[fresh13
        as usize] = ((*ctx).tag.b[fresh13 as usize] as libc::c_ulong
        ^ authlen >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
}
pub unsafe extern "C" fn nettle_ccm_update(
    mut ctx: *mut ccm_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut end: *const uint8_t = data.offset(length as isize);
    if ((*ctx).blength as libc::c_ulong).wrapping_add(length)
        < 16 as libc::c_int as libc::c_ulong
    {
        nettle_memxor(
            &mut *((*ctx).tag.b).as_mut_ptr().offset((*ctx).blength as isize)
                as *mut uint8_t as *mut libc::c_void,
            data as *const libc::c_void,
            length,
        );
        (*ctx)
            .blength = ((*ctx).blength as libc::c_ulong).wrapping_add(length)
            as libc::c_uint as libc::c_uint;
        return;
    }
    if (*ctx).blength != 0 {
        nettle_memxor(
            &mut *((*ctx).tag.b).as_mut_ptr().offset((*ctx).blength as isize)
                as *mut uint8_t as *mut libc::c_void,
            data as *const libc::c_void,
            (16 as libc::c_int as libc::c_uint).wrapping_sub((*ctx).blength) as size_t,
        );
        data = data
            .offset(
                (16 as libc::c_int as libc::c_uint).wrapping_sub((*ctx).blength) as isize,
            );
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            ((*ctx).tag.b).as_mut_ptr(),
            ((*ctx).tag.b).as_mut_ptr(),
        );
    }
    while data.offset(16 as libc::c_int as isize) < end {
        nettle_memxor(
            ((*ctx).tag.b).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            ((*ctx).tag.b).as_mut_ptr(),
            ((*ctx).tag.b).as_mut_ptr(),
        );
        data = data.offset(16 as libc::c_int as isize);
    }
    (*ctx).blength = end.offset_from(data) as libc::c_long as libc::c_uint;
    if (*ctx).blength != 0 {
        nettle_memxor(
            &mut (*ctx).tag.b as *mut [uint8_t; 16] as *mut libc::c_void,
            data as *const libc::c_void,
            (*ctx).blength as size_t,
        );
    }
}
pub unsafe extern "C" fn nettle_ccm_encrypt(
    mut ctx: *mut ccm_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    ccm_pad(ctx, cipher, f);
    nettle_ccm_update(ctx, cipher, f, length, src);
    nettle_ctr_crypt(
        cipher,
        f,
        16 as libc::c_int as size_t,
        ((*ctx).ctr.b).as_mut_ptr(),
        length,
        dst,
        src,
    );
}
pub unsafe extern "C" fn nettle_ccm_decrypt(
    mut ctx: *mut ccm_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ctr_crypt(
        cipher,
        f,
        16 as libc::c_int as size_t,
        ((*ctx).ctr.b).as_mut_ptr(),
        length,
        dst,
        src,
    );
    ccm_pad(ctx, cipher, f);
    nettle_ccm_update(ctx, cipher, f, length, dst);
}
pub unsafe extern "C" fn nettle_ccm_digest(
    mut ctx: *mut ccm_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut i: libc::c_int = 16 as libc::c_int
        - (((*ctx).ctr.b[0 as libc::c_int as usize] as libc::c_int & 0x7 as libc::c_int)
            + 1 as libc::c_int);
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= CCM_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
            b"ccm.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void nettle_ccm_digest(struct ccm_ctx *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2757: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= CCM_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"ccm.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void nettle_ccm_digest(struct ccm_ctx *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < 16 as libc::c_int {
        let fresh14 = i;
        i = i + 1;
        (*ctx).ctr.b[fresh14 as usize] = 0 as libc::c_int as uint8_t;
    }
    ccm_pad(ctx, cipher, f);
    nettle_ctr_crypt(
        cipher,
        f,
        16 as libc::c_int as size_t,
        ((*ctx).ctr.b).as_mut_ptr(),
        length,
        digest,
        ((*ctx).tag.b).as_mut_ptr(),
    );
}
pub unsafe extern "C" fn nettle_ccm_encrypt_message(
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: ccm_ctx = ccm_ctx {
        ctr: nettle_block16 { b: [0; 16] },
        tag: nettle_block16 { b: [0; 16] },
        blength: 0,
    };
    let mut tag: *mut uint8_t = dst.offset(clength.wrapping_sub(tlength) as isize);
    if clength >= tlength {} else {
        __assert_fail(
            b"clength >= tlength\0" as *const u8 as *const libc::c_char,
            b"ccm.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 162],
                &[libc::c_char; 162],
            >(
                b"void nettle_ccm_encrypt_message(const void *, nettle_cipher_func *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2900: {
        if clength >= tlength {} else {
            __assert_fail(
                b"clength >= tlength\0" as *const u8 as *const libc::c_char,
                b"ccm.c\0" as *const u8 as *const libc::c_char,
                242 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 162],
                    &[libc::c_char; 162],
                >(
                    b"void nettle_ccm_encrypt_message(const void *, nettle_cipher_func *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_ccm_set_nonce(
        &mut ctx,
        cipher,
        f,
        nlength,
        nonce,
        alength,
        clength.wrapping_sub(tlength),
        tlength,
    );
    nettle_ccm_update(&mut ctx, cipher, f, alength, adata);
    nettle_ccm_encrypt(&mut ctx, cipher, f, clength.wrapping_sub(tlength), dst, src);
    nettle_ccm_digest(&mut ctx, cipher, f, tlength, tag);
}
pub unsafe extern "C" fn nettle_ccm_decrypt_message(
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> libc::c_int {
    let mut ctx: ccm_ctx = ccm_ctx {
        ctr: nettle_block16 { b: [0; 16] },
        tag: nettle_block16 { b: [0; 16] },
        blength: 0,
    };
    let mut tag: [uint8_t; 16] = [0; 16];
    nettle_ccm_set_nonce(&mut ctx, cipher, f, nlength, nonce, alength, mlength, tlength);
    nettle_ccm_update(&mut ctx, cipher, f, alength, adata);
    nettle_ccm_decrypt(&mut ctx, cipher, f, mlength, dst, src);
    nettle_ccm_digest(&mut ctx, cipher, f, tlength, tag.as_mut_ptr());
    return nettle_memeql_sec(
        tag.as_mut_ptr() as *const libc::c_void,
        src.offset(mlength as isize) as *const libc::c_void,
        tlength,
    );
}
