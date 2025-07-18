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
    fn _nettle_ctr_crypt16(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        fill: Option::<nettle_fill16_func>,
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
pub type nettle_fill16_func = unsafe extern "C" fn(
    *mut uint8_t,
    size_t,
    *mut nettle_block16,
) -> ();
unsafe extern "C" fn ctr_fill(
    mut block_size: size_t,
    mut ctr: *mut uint8_t,
    mut length: size_t,
    mut buffer: *mut uint8_t,
) -> size_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i.wrapping_add(block_size) <= length {
        memcpy(
            buffer.offset(i as isize) as *mut libc::c_void,
            ctr as *const libc::c_void,
            block_size,
        );
        let mut increment_i: libc::c_uint = block_size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
        let ref mut fresh0 = *ctr.offset(increment_i as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        if *fresh0 as libc::c_int == 0 as libc::c_int {
            while increment_i > 0 as libc::c_int as libc::c_uint
                && {
                    increment_i = increment_i.wrapping_sub(1);
                    let ref mut fresh1 = *ctr.offset(increment_i as isize);
                    *fresh1 = (*fresh1).wrapping_add(1);
                    *fresh1 as libc::c_int == 0 as libc::c_int
                }
            {}
        }
        i = (i as libc::c_ulong).wrapping_add(block_size) as size_t as size_t;
    }
    return i;
}
unsafe extern "C" fn ctr_fill16(
    mut ctr: *mut uint8_t,
    mut blocks: size_t,
    mut buffer: *mut nettle_block16,
) {
    let mut hi: uint64_t = 0;
    let mut lo: uint64_t = 0;
    let mut i: size_t = 0;
    hi = (*ctr.offset(7 as libc::c_int as isize) as uint64_t) << 56 as libc::c_int
        | (*ctr.offset(6 as libc::c_int as isize) as uint64_t) << 48 as libc::c_int
        | (*ctr.offset(5 as libc::c_int as isize) as uint64_t) << 40 as libc::c_int
        | (*ctr.offset(4 as libc::c_int as isize) as uint64_t) << 32 as libc::c_int
        | (*ctr.offset(3 as libc::c_int as isize) as uint64_t) << 24 as libc::c_int
        | (*ctr.offset(2 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
        | (*ctr.offset(1 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
        | *ctr.offset(0 as libc::c_int as isize) as uint64_t;
    lo = (*ctr.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint64_t) << 56 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint64_t) << 48 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint64_t) << 40 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint64_t) << 32 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(4 as libc::c_int as isize)
            as uint64_t) << 24 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(5 as libc::c_int as isize)
            as uint64_t) << 16 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(6 as libc::c_int as isize)
            as uint64_t) << 8 as libc::c_int
        | *ctr.offset(8 as libc::c_int as isize).offset(7 as libc::c_int as isize)
            as uint64_t;
    i = 0 as libc::c_int as size_t;
    while i < blocks {
        (*buffer.offset(i as isize)).u64_0[0 as libc::c_int as usize] = hi;
        (*buffer.offset(i as isize)).u64_0[1 as libc::c_int as usize] = lo.swap_bytes();
        lo = lo.wrapping_add(1);
        if lo == 0 {
            hi = hi
                .swap_bytes()
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .swap_bytes();
        }
        i = i.wrapping_add(1);
        i;
    }
    *ctr
        .offset(
            7 as libc::c_int as isize,
        ) = (hi >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            6 as libc::c_int as isize,
        ) = (hi >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            5 as libc::c_int as isize,
        ) = (hi >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            4 as libc::c_int as isize,
        ) = (hi >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            3 as libc::c_int as isize,
        ) = (hi >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            2 as libc::c_int as isize,
        ) = (hi >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            1 as libc::c_int as isize,
        ) = (hi >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(
            0 as libc::c_int as isize,
        ) = (hi & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (lo >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (lo >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (lo >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (lo >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (lo >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (lo >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (lo >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *ctr
        .offset(8 as libc::c_int as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (lo & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
}
pub unsafe extern "C" fn nettle_ctr_crypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut ctr: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if block_size == 16 as libc::c_int as libc::c_ulong {
        _nettle_ctr_crypt16(
            ctx,
            f,
            Some(ctr_fill16 as nettle_fill16_func),
            ctr,
            length,
            dst,
            src,
        );
        return;
    }
    if src != dst as *const uint8_t {
        let mut filled: size_t = ctr_fill(block_size, ctr, length, dst);
        f.unwrap()(ctx, filled, dst, dst);
        nettle_memxor(dst as *mut libc::c_void, src as *const libc::c_void, filled);
        if filled < length {
            let mut block: *mut uint8_t = 0 as *mut uint8_t;
            let mut fresh2 = ::std::vec::from_elem(
                0,
                (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                    .wrapping_mul(block_size) as usize,
            );
            block = fresh2.leak().as_mut_ptr() as *mut uint8_t;
            f.unwrap()(ctx, block_size, block, ctr);
            let mut increment_i: libc::c_uint = block_size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
            let ref mut fresh3 = *ctr.offset(increment_i as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            if *fresh3 as libc::c_int == 0 as libc::c_int {
                while increment_i > 0 as libc::c_int as libc::c_uint
                    && {
                        increment_i = increment_i.wrapping_sub(1);
                        let ref mut fresh4 = *ctr.offset(increment_i as isize);
                        *fresh4 = (*fresh4).wrapping_add(1);
                        *fresh4 as libc::c_int == 0 as libc::c_int
                    }
                {}
            }
            nettle_memxor3(
                dst.offset(filled as isize) as *mut libc::c_void,
                src.offset(filled as isize) as *const libc::c_void,
                block as *const libc::c_void,
                length.wrapping_sub(filled),
            );
        }
    } else {
        let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
        let mut buffer_size: size_t = 0;
        if length < block_size {
            buffer_size = block_size;
        } else if length <= 512 as libc::c_int as libc::c_ulong {
            buffer_size = length;
        } else {
            buffer_size = 512 as libc::c_int as size_t;
        }
        let mut fresh5 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(buffer_size)
                as usize,
        );
        buffer = fresh5.leak().as_mut_ptr() as *mut uint8_t;
        while length >= block_size {
            let mut filled_0: size_t = ctr_fill(
                block_size,
                ctr,
                if buffer_size < length { buffer_size } else { length },
                buffer,
            );
            if filled_0 > 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"filled > 0\0" as *const u8 as *const libc::c_char,
                    b"ctr.c\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 113],
                        &[libc::c_char; 113],
                    >(
                        b"void nettle_ctr_crypt(const void *, nettle_cipher_func *, size_t, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1808: {
                if filled_0 > 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"filled > 0\0" as *const u8 as *const libc::c_char,
                        b"ctr.c\0" as *const u8 as *const libc::c_char,
                        162 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"void nettle_ctr_crypt(const void *, nettle_cipher_func *, size_t, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            f.unwrap()(ctx, filled_0, buffer, buffer);
            nettle_memxor(
                dst as *mut libc::c_void,
                buffer as *const libc::c_void,
                filled_0,
            );
            length = (length as libc::c_ulong).wrapping_sub(filled_0) as size_t
                as size_t;
            dst = dst.offset(filled_0 as isize);
        }
        if length > 0 as libc::c_int as libc::c_ulong {
            f.unwrap()(ctx, block_size, buffer, ctr);
            let mut increment_i_0: libc::c_uint = block_size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
            let ref mut fresh6 = *ctr.offset(increment_i_0 as isize);
            *fresh6 = (*fresh6).wrapping_add(1);
            if *fresh6 as libc::c_int == 0 as libc::c_int {
                while increment_i_0 > 0 as libc::c_int as libc::c_uint
                    && {
                        increment_i_0 = increment_i_0.wrapping_sub(1);
                        let ref mut fresh7 = *ctr.offset(increment_i_0 as isize);
                        *fresh7 = (*fresh7).wrapping_add(1);
                        *fresh7 as libc::c_int == 0 as libc::c_int
                    }
                {}
            }
            nettle_memxor(
                dst as *mut libc::c_void,
                buffer as *const libc::c_void,
                length,
            );
        }
    };
}
