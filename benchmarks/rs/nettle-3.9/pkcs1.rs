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
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub unsafe extern "C" fn _nettle_pkcs1_signature_prefix(
    mut key_size: libc::c_uint,
    mut buffer: *mut uint8_t,
    mut id_size: libc::c_uint,
    mut id: *const uint8_t,
    mut digest_size: libc::c_uint,
) -> *mut uint8_t {
    let mut j: libc::c_uint = 0;
    if key_size
        < (11 as libc::c_int as libc::c_uint)
            .wrapping_add(id_size)
            .wrapping_add(digest_size)
    {
        return 0 as *mut uint8_t;
    }
    j = key_size.wrapping_sub(digest_size).wrapping_sub(id_size);
    memcpy(
        buffer.offset(j as isize) as *mut libc::c_void,
        id as *const libc::c_void,
        id_size as libc::c_ulong,
    );
    *buffer.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *buffer.offset(1 as libc::c_int as isize) = 1 as libc::c_int as uint8_t;
    *buffer
        .offset(
            j.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = 0 as libc::c_int as uint8_t;
    if j >= 11 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"j >= 11\0" as *const u8 as *const libc::c_char,
            b"pkcs1.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"uint8_t *_nettle_pkcs1_signature_prefix(unsigned int, uint8_t *, unsigned int, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3718: {
        if j >= 11 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"j >= 11\0" as *const u8 as *const libc::c_char,
                b"pkcs1.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"uint8_t *_nettle_pkcs1_signature_prefix(unsigned int, uint8_t *, unsigned int, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memset(
        buffer.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        0xff as libc::c_int,
        j.wrapping_sub(3 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    return buffer.offset(j as isize).offset(id_size as isize);
}
