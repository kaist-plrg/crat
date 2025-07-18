use ::libc;
extern "C" {
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
pub unsafe extern "C" fn nettle_cfb_encrypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(block_size)
            as usize,
    );
    buffer = fresh0.leak().as_mut_ptr() as *mut uint8_t;
    if src != dst as *const uint8_t {
        p = iv;
        while length >= block_size {
            f.unwrap()(ctx, block_size, dst, p);
            nettle_memxor(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                block_size,
            );
            p = dst;
            dst = dst.offset(block_size as isize);
            src = src.offset(block_size as isize);
            length = (length as libc::c_ulong).wrapping_sub(block_size) as size_t
                as size_t;
        }
    } else {
        p = iv;
        while length >= block_size {
            f.unwrap()(ctx, block_size, buffer, p);
            nettle_memxor(
                dst as *mut libc::c_void,
                buffer as *const libc::c_void,
                block_size,
            );
            p = dst;
            dst = dst.offset(block_size as isize);
            src = src.offset(block_size as isize);
            length = (length as libc::c_ulong).wrapping_sub(block_size) as size_t
                as size_t;
        }
    }
    if p != iv {
        memcpy(iv as *mut libc::c_void, p as *const libc::c_void, block_size);
    }
    if length != 0 {
        f.unwrap()(ctx, block_size, buffer, iv);
        nettle_memxor3(
            dst as *mut libc::c_void,
            buffer as *const libc::c_void,
            src as *const libc::c_void,
            length,
        );
    }
}
pub unsafe extern "C" fn nettle_cfb_decrypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if src != dst as *const uint8_t {
        let mut left: size_t = length.wrapping_rem(block_size);
        length = (length as libc::c_ulong).wrapping_sub(left) as size_t as size_t;
        if length > 0 as libc::c_int as libc::c_ulong {
            f.unwrap()(ctx, block_size, dst, iv);
            f
                .unwrap()(
                ctx,
                length.wrapping_sub(block_size),
                dst.offset(block_size as isize),
                src,
            );
            memcpy(
                iv as *mut libc::c_void,
                src.offset(length as isize).offset(-(block_size as isize))
                    as *const libc::c_void,
                block_size,
            );
            nettle_memxor(dst as *mut libc::c_void, src as *const libc::c_void, length);
        }
        if left > 0 as libc::c_int as libc::c_ulong {
            let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
            let mut fresh1 = ::std::vec::from_elem(
                0,
                (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                    .wrapping_mul(block_size) as usize,
            );
            buffer = fresh1.leak().as_mut_ptr() as *mut uint8_t;
            f.unwrap()(ctx, block_size, buffer, iv);
            nettle_memxor3(
                dst.offset(length as isize) as *mut libc::c_void,
                src.offset(length as isize) as *const libc::c_void,
                buffer as *const libc::c_void,
                left,
            );
        }
    } else {
        let mut buffer_0: *mut uint8_t = 0 as *mut uint8_t;
        let mut initial_iv: *mut uint8_t = 0 as *mut uint8_t;
        let mut buffer_size: size_t = 0;
        let mut left_0: size_t = 0;
        buffer_size = (512 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                (512 as libc::c_int as libc::c_ulong).wrapping_rem(block_size),
            );
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(buffer_size)
                as usize,
        );
        buffer_0 = fresh2.leak().as_mut_ptr() as *mut uint8_t;
        let mut fresh3 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(block_size)
                as usize,
        );
        initial_iv = fresh3.leak().as_mut_ptr() as *mut uint8_t;
        left_0 = length.wrapping_rem(block_size);
        length = (length as libc::c_ulong).wrapping_sub(left_0) as size_t as size_t;
        while length > 0 as libc::c_int as libc::c_ulong {
            let mut part: size_t = if length > buffer_size {
                buffer_size
            } else {
                length
            };
            f.unwrap()(ctx, block_size, buffer_0, iv);
            f
                .unwrap()(
                ctx,
                part.wrapping_sub(block_size),
                buffer_0.offset(block_size as isize),
                dst,
            );
            memcpy(
                iv as *mut libc::c_void,
                dst.offset(part as isize).offset(-(block_size as isize))
                    as *const libc::c_void,
                block_size,
            );
            nettle_memxor(
                dst as *mut libc::c_void,
                buffer_0 as *const libc::c_void,
                part,
            );
            length = (length as libc::c_ulong).wrapping_sub(part) as size_t as size_t;
            dst = dst.offset(part as isize);
        }
        if left_0 > 0 as libc::c_int as libc::c_ulong {
            f.unwrap()(ctx, block_size, buffer_0, iv);
            nettle_memxor(
                dst as *mut libc::c_void,
                buffer_0 as *const libc::c_void,
                left_0,
            );
        }
    };
}
pub unsafe extern "C" fn nettle_cfb8_encrypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut outbuf: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(block_size.wrapping_mul(2 as libc::c_int as libc::c_ulong))
            as usize,
    );
    buffer = fresh4.leak().as_mut_ptr() as *mut uint8_t;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(block_size)
            as usize,
    );
    outbuf = fresh5.leak().as_mut_ptr() as *mut uint8_t;
    let mut pos: uint8_t = 0;
    memcpy(buffer as *mut libc::c_void, iv as *const libc::c_void, block_size);
    pos = 0 as libc::c_int as uint8_t;
    while length != 0 {
        let mut t: uint8_t = 0;
        if pos as libc::c_ulong == block_size {
            memcpy(
                buffer as *mut libc::c_void,
                buffer.offset(block_size as isize) as *const libc::c_void,
                block_size,
            );
            pos = 0 as libc::c_int as uint8_t;
        }
        f.unwrap()(ctx, block_size, outbuf, buffer.offset(pos as libc::c_int as isize));
        let fresh6 = src;
        src = src.offset(1);
        let fresh7 = dst;
        dst = dst.offset(1);
        *fresh7 = (*fresh6 as libc::c_int
            ^ *outbuf.offset(0 as libc::c_int as isize) as libc::c_int) as uint8_t;
        t = *fresh7;
        *buffer.offset((pos as libc::c_ulong).wrapping_add(block_size) as isize) = t;
        length = length.wrapping_sub(1);
        length;
        pos = pos.wrapping_add(1);
        pos;
    }
    memcpy(
        iv as *mut libc::c_void,
        buffer.offset(pos as libc::c_int as isize) as *const libc::c_void,
        block_size,
    );
}
pub unsafe extern "C" fn nettle_cfb8_decrypt(
    mut ctx: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut block_size: size_t,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut outbuf: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(block_size.wrapping_mul(2 as libc::c_int as libc::c_ulong))
            as usize,
    );
    buffer = fresh8.leak().as_mut_ptr() as *mut uint8_t;
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(block_size.wrapping_mul(2 as libc::c_int as libc::c_ulong))
            as usize,
    );
    outbuf = fresh9.leak().as_mut_ptr() as *mut uint8_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    memcpy(buffer as *mut libc::c_void, iv as *const libc::c_void, block_size);
    memcpy(
        buffer.offset(block_size as isize) as *mut libc::c_void,
        src as *const libc::c_void,
        if length < block_size { length } else { block_size },
    );
    while length != 0 {
        i = 0 as libc::c_int as uint8_t;
        while (i as libc::c_ulong) < length && (i as libc::c_ulong) < block_size {
            f
                .unwrap()(
                ctx,
                block_size,
                outbuf.offset(i as libc::c_int as isize),
                buffer.offset(i as libc::c_int as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        nettle_memxor3(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            outbuf as *const libc::c_void,
            i as size_t,
        );
        length = (length as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as size_t
            as size_t;
        src = src.offset(i as libc::c_int as isize);
        dst = dst.offset(i as libc::c_int as isize);
        if i as libc::c_ulong == block_size {
            memcpy(
                buffer as *mut libc::c_void,
                buffer.offset(block_size as isize) as *const libc::c_void,
                block_size,
            );
            memcpy(
                buffer.offset(block_size as isize) as *mut libc::c_void,
                src as *const libc::c_void,
                if length < block_size { length } else { block_size },
            );
        }
    }
    memcpy(
        iv as *mut libc::c_void,
        buffer.offset(i as libc::c_int as isize) as *const libc::c_void,
        block_size,
    );
}
