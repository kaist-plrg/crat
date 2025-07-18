use ::libc;
extern "C" {
    fn nettle_cnd_memcpy(
        cnd: libc::c_int,
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub unsafe extern "C" fn _nettle_pkcs1_sec_decrypt(
    mut length: size_t,
    mut message: *mut uint8_t,
    mut padded_message_length: size_t,
    mut padded_message: *const uint8_t,
) -> libc::c_int {
    let mut ok: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut t: size_t = 0;
    if length.wrapping_add(11 as libc::c_int as libc::c_ulong) > padded_message_length {
        return 0 as libc::c_int;
    }
    t = padded_message_length
        .wrapping_sub(length)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        ((*padded_message.offset(0 as libc::c_int as isize) as uint32_t
            ^ 0 as libc::c_int as uint32_t)
            .wrapping_sub(1 as libc::c_uint) >> 31 as libc::c_int) as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
            as libc::c_uint
            & (*padded_message.offset(1 as libc::c_int as isize) as uint32_t
                ^ 2 as libc::c_int as uint32_t)
                .wrapping_sub(1 as libc::c_uint) >> 31 as libc::c_int) as libc::c_int
            as libc::c_int,
    );
    i = 2 as libc::c_int as size_t;
    while i < t {
        ::std::ptr::write_volatile(
            &mut ok as *mut libc::c_int,
            (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
                as libc::c_uint
                & (0 as libc::c_uint)
                    .wrapping_sub(
                        *padded_message.offset(i as isize) as uint32_t
                            ^ 0 as libc::c_int as uint32_t,
                    ) >> 31 as libc::c_int) as libc::c_int as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
            as libc::c_uint
            & (*padded_message.offset(t as isize) as uint32_t
                ^ 0 as libc::c_int as uint32_t)
                .wrapping_sub(1 as libc::c_uint) >> 31 as libc::c_int) as libc::c_int
            as libc::c_int,
    );
    nettle_cnd_memcpy(
        ok,
        message as *mut libc::c_void,
        padded_message.offset(t as isize).offset(1 as libc::c_int as isize)
            as *const libc::c_void,
        length,
    );
    return ok;
}
pub unsafe extern "C" fn _nettle_pkcs1_sec_decrypt_variable(
    mut length: *mut size_t,
    mut message: *mut uint8_t,
    mut padded_message_length: size_t,
    mut padded_message: *const uint8_t,
) -> libc::c_int {
    let mut not_found: libc::c_int = 1 as libc::c_int;
    let mut ok: libc::c_int = 0;
    let mut offset: size_t = 0;
    let mut buflen: size_t = 0;
    let mut msglen: size_t = 0;
    let mut shift: size_t = 0;
    let mut i: size_t = 0;
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        ((*padded_message.offset(0 as libc::c_int as isize) as uint32_t
            ^ 0 as libc::c_int as uint32_t)
            .wrapping_sub(1 as libc::c_uint) >> 31 as libc::c_int) as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
            as libc::c_uint
            & (*padded_message.offset(1 as libc::c_int as isize) as uint32_t
                ^ 2 as libc::c_int as uint32_t)
                .wrapping_sub(1 as libc::c_uint) >> 31 as libc::c_int) as libc::c_int
            as libc::c_int,
    );
    ::std::ptr::write_volatile(&mut offset as *mut size_t, 3 as libc::c_int as size_t);
    i = 2 as libc::c_int as size_t;
    while i < padded_message_length {
        ::std::ptr::write_volatile(
            &mut not_found as *mut libc::c_int,
            (::std::ptr::read_volatile::<libc::c_int>(&not_found as *const libc::c_int)
                as libc::c_uint
                & (0 as libc::c_uint)
                    .wrapping_sub(
                        *padded_message.offset(i as isize) as uint32_t
                            ^ 0 as libc::c_int as uint32_t,
                    ) >> 31 as libc::c_int) as libc::c_int as libc::c_int,
        );
        ::std::ptr::write_volatile(
            &mut offset as *mut size_t,
            (::std::ptr::read_volatile::<size_t>(&offset as *const size_t)
                as libc::c_ulong)
                .wrapping_add(not_found as libc::c_ulong) as size_t as size_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
            as libc::c_uint
            & (0 as libc::c_uint)
                .wrapping_sub(not_found as uint32_t ^ 1 as libc::c_int as uint32_t)
                >> 31 as libc::c_int) as libc::c_int as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
            as libc::c_uint
            & (1 as libc::c_uint)
                .wrapping_sub(
                    (offset as uint32_t).wrapping_sub(11 as libc::c_int as uint32_t)
                        >> 31 as libc::c_int,
                )) as libc::c_int as libc::c_int,
    );
    msglen = padded_message_length.wrapping_sub(offset);
    buflen = *length;
    if buflen > padded_message_length {
        buflen = padded_message_length;
    }
    ::std::ptr::write_volatile(
        &mut ok as *mut libc::c_int,
        (::std::ptr::read_volatile::<libc::c_int>(&ok as *const libc::c_int)
            as libc::c_uint
            & (1 as libc::c_uint)
                .wrapping_sub(
                    (buflen as uint32_t).wrapping_sub(msglen as uint32_t)
                        >> 31 as libc::c_int,
                )) as libc::c_int as libc::c_int,
    );
    shift = padded_message_length.wrapping_sub(buflen);
    nettle_cnd_memcpy(
        ok,
        message as *mut libc::c_void,
        padded_message.offset(shift as isize) as *const libc::c_void,
        buflen,
    );
    ::std::ptr::write_volatile(
        &mut offset as *mut size_t,
        (::std::ptr::read_volatile::<size_t>(&offset as *const size_t) as libc::c_ulong)
            .wrapping_sub(shift) as size_t as size_t,
    );
    shift = 1 as libc::c_int as size_t;
    while shift < buflen {
        nettle_cnd_memcpy(
            (offset & ok as libc::c_ulong) as libc::c_int,
            message as *mut libc::c_void,
            message.offset(shift as isize) as *const libc::c_void,
            buflen.wrapping_sub(shift),
        );
        shift <<= 1 as libc::c_int;
        ::std::ptr::write_volatile(
            &mut offset as *mut size_t,
            ::std::ptr::read_volatile::<size_t>(&offset as *const size_t)
                >> 1 as libc::c_int,
        );
    }
    *length = (msglen & (ok as size_t).wrapping_neg())
        .wrapping_add(
            *length & (ok as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    return ok;
}
