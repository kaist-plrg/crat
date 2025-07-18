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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub unsafe extern "C" fn nettle_pbkdf2(
    mut mac_ctx: *mut libc::c_void,
    mut update: Option::<nettle_hash_update_func>,
    mut digest: Option::<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut iterations: libc::c_uint,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut U: *mut uint8_t = 0 as *mut uint8_t;
    let mut T: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: libc::c_uint = 0;
    if iterations > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"iterations > 0\0" as *const u8 as *const libc::c_char,
            b"pbkdf2.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 147],
                &[libc::c_char; 147],
            >(
                b"void nettle_pbkdf2(void *, nettle_hash_update_func *, nettle_hash_digest_func *, size_t, unsigned int, size_t, const uint8_t *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2008: {
        if iterations > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"iterations > 0\0" as *const u8 as *const libc::c_char,
                b"pbkdf2.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 147],
                    &[libc::c_char; 147],
                >(
                    b"void nettle_pbkdf2(void *, nettle_hash_update_func *, nettle_hash_digest_func *, size_t, unsigned int, size_t, const uint8_t *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(digest_size)
            as usize,
    );
    U = fresh0.leak().as_mut_ptr() as *mut uint8_t;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(digest_size)
            as usize,
    );
    T = fresh1.leak().as_mut_ptr() as *mut uint8_t;
    i = 1 as libc::c_int as libc::c_uint;
    loop {
        let mut tmp: [uint8_t; 4] = [0; 4];
        let mut prev: *mut uint8_t = 0 as *mut uint8_t;
        let mut u: libc::c_uint = 0;
        tmp[0 as libc::c_int
            as usize] = (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        tmp[1 as libc::c_int
            as usize] = (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        tmp[2 as libc::c_int
            as usize] = (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        tmp[3 as libc::c_int
            as usize] = (i & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        update.unwrap()(mac_ctx, salt_length, salt);
        update
            .unwrap()(
            mac_ctx,
            ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
            tmp.as_mut_ptr(),
        );
        digest.unwrap()(mac_ctx, digest_size, T);
        prev = T;
        u = 1 as libc::c_int as libc::c_uint;
        while u < iterations {
            update.unwrap()(mac_ctx, digest_size, prev);
            digest.unwrap()(mac_ctx, digest_size, U);
            nettle_memxor(T as *mut libc::c_void, U as *const libc::c_void, digest_size);
            u = u.wrapping_add(1);
            u;
            prev = U;
        }
        if length <= digest_size {
            memcpy(dst as *mut libc::c_void, T as *const libc::c_void, length);
            return;
        }
        memcpy(dst as *mut libc::c_void, T as *const libc::c_void, digest_size);
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(digest_size as isize);
        length = (length as libc::c_ulong).wrapping_sub(digest_size) as size_t as size_t;
    };
}
