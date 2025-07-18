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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
pub unsafe extern "C" fn nettle_hmac_set_key(
    mut outer: *mut libc::c_void,
    mut inner: *mut libc::c_void,
    mut state: *mut libc::c_void,
    mut hash: *const nettle_hash,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    let mut pad: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul((*hash).block_size as libc::c_ulong) as usize,
    );
    pad = fresh0.leak().as_mut_ptr() as *mut uint8_t;
    ((*hash).init).unwrap()(outer);
    ((*hash).init).unwrap()(inner);
    if key_length > (*hash).block_size as libc::c_ulong {
        let mut digest: *mut uint8_t = 0 as *mut uint8_t;
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul((*hash).digest_size as libc::c_ulong) as usize,
        );
        digest = fresh1.leak().as_mut_ptr() as *mut uint8_t;
        ((*hash).init).unwrap()(state);
        ((*hash).update).unwrap()(state, key_length, key);
        ((*hash).digest).unwrap()(state, (*hash).digest_size as size_t, digest);
        key = digest;
        key_length = (*hash).digest_size as size_t;
    }
    if key_length <= (*hash).block_size as libc::c_ulong {} else {
        __assert_fail(
            b"key_length <= hash->block_size\0" as *const u8 as *const libc::c_char,
            b"hmac.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"void nettle_hmac_set_key(void *, void *, void *, const struct nettle_hash *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1016: {
        if key_length <= (*hash).block_size as libc::c_ulong {} else {
            __assert_fail(
                b"key_length <= hash->block_size\0" as *const u8 as *const libc::c_char,
                b"hmac.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 102],
                    &[libc::c_char; 102],
                >(
                    b"void nettle_hmac_set_key(void *, void *, void *, const struct nettle_hash *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memset(
        pad as *mut libc::c_void,
        0x5c as libc::c_int,
        (*hash).block_size as libc::c_ulong,
    );
    nettle_memxor(pad as *mut libc::c_void, key as *const libc::c_void, key_length);
    ((*hash).update).unwrap()(outer, (*hash).block_size as size_t, pad);
    memset(
        pad as *mut libc::c_void,
        0x36 as libc::c_int,
        (*hash).block_size as libc::c_ulong,
    );
    nettle_memxor(pad as *mut libc::c_void, key as *const libc::c_void, key_length);
    ((*hash).update).unwrap()(inner, (*hash).block_size as size_t, pad);
    memcpy(state, inner, (*hash).context_size as libc::c_ulong);
}
pub unsafe extern "C" fn nettle_hmac_update(
    mut state: *mut libc::c_void,
    mut hash: *const nettle_hash,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    ((*hash).update).unwrap()(state, length, data);
}
pub unsafe extern "C" fn nettle_hmac_digest(
    mut outer: *const libc::c_void,
    mut inner: *const libc::c_void,
    mut state: *mut libc::c_void,
    mut hash: *const nettle_hash,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut digest: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul((*hash).digest_size as libc::c_ulong) as usize,
    );
    digest = fresh2.leak().as_mut_ptr() as *mut uint8_t;
    ((*hash).digest).unwrap()(state, (*hash).digest_size as size_t, digest);
    memcpy(state, outer, (*hash).context_size as libc::c_ulong);
    ((*hash).update).unwrap()(state, (*hash).digest_size as size_t, digest);
    ((*hash).digest).unwrap()(state, length, dst);
    memcpy(state, inner, (*hash).context_size as libc::c_ulong);
}
