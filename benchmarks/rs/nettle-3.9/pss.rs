use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_mpz_get_str_256(length: size_t, s: *mut uint8_t, x: *const __mpz_struct);
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn nettle_pss_mgf1(
        seed: *const libc::c_void,
        hash: *const nettle_hash,
        length: size_t,
        mask: *mut uint8_t,
    );
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
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
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_srcptr = *const __mpz_struct;
static mut pss_masks: [uint8_t; 8] = [
    0xff as libc::c_int as uint8_t,
    0x7f as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0x1f as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
];
static mut pss_pad: [uint8_t; 8] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
pub unsafe extern "C" fn nettle_pss_encode_mgf1(
    mut m: *mut __mpz_struct,
    mut bits: size_t,
    mut hash: *const nettle_hash,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut digest: *const uint8_t,
) -> libc::c_int {
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut key_size: size_t = bits
        .wrapping_add(7 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    let mut j: size_t = 0;
    tmp_em_size = key_size;
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(key_size),
    ) as *mut uint8_t;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (*hash).context_size as libc::c_ulong as usize,
    );
    state = fresh0.leak().as_mut_ptr() as *mut _;
    if key_size
        < ((*hash).digest_size as libc::c_ulong)
            .wrapping_add(salt_length)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
    {
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 0 as libc::c_int;
    }
    ((*hash).init).unwrap()(state);
    ((*hash).update)
        .unwrap()(
        state,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
        pss_pad.as_ptr(),
    );
    ((*hash).update).unwrap()(state, (*hash).digest_size as size_t, digest);
    ((*hash).update).unwrap()(state, salt_length, salt);
    ((*hash).digest)
        .unwrap()(
        state,
        (*hash).digest_size as size_t,
        em
            .offset(key_size as isize)
            .offset(-((*hash).digest_size as isize))
            .offset(-(1 as libc::c_int as isize)),
    );
    ((*hash).init).unwrap()(state);
    ((*hash).update)
        .unwrap()(
        state,
        (*hash).digest_size as size_t,
        em
            .offset(key_size as isize)
            .offset(-((*hash).digest_size as isize))
            .offset(-(1 as libc::c_int as isize)),
    );
    nettle_pss_mgf1(
        state,
        hash,
        key_size
            .wrapping_sub((*hash).digest_size as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        em,
    );
    j = key_size
        .wrapping_sub(salt_length)
        .wrapping_sub((*hash).digest_size as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong);
    let fresh1 = j;
    j = j.wrapping_add(1);
    let ref mut fresh2 = *em.offset(fresh1 as isize);
    *fresh2 = (*fresh2 as libc::c_int ^ 1 as libc::c_int) as uint8_t;
    nettle_memxor(
        em.offset(j as isize) as *mut libc::c_void,
        salt as *const libc::c_void,
        salt_length,
    );
    j = (j as libc::c_ulong).wrapping_add(salt_length) as size_t as size_t;
    j = (j as libc::c_ulong).wrapping_add((*hash).digest_size as libc::c_ulong) as size_t
        as size_t;
    *em.offset(j as isize) = 0xbc as libc::c_int as uint8_t;
    *em = (*em as libc::c_int
        & pss_masks[(8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(key_size)
            .wrapping_sub(bits) as usize] as libc::c_int) as uint8_t;
    nettle_mpz_set_str_256_u(m, key_size, em);
    _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_pss_verify_mgf1(
    mut m: *const __mpz_struct,
    mut bits: size_t,
    mut hash: *const nettle_hash,
    mut salt_length: size_t,
    mut digest: *const uint8_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    let mut h2: *mut uint8_t = 0 as *mut uint8_t;
    let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut h: *mut uint8_t = 0 as *mut uint8_t;
    let mut db: *mut uint8_t = 0 as *mut uint8_t;
    let mut salt: *mut uint8_t = 0 as *mut uint8_t;
    let mut key_size: size_t = bits
        .wrapping_add(7 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    let mut j: size_t = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    tmp_em_size = key_size.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(key_size.wrapping_mul(2 as libc::c_int as libc::c_ulong)),
    ) as *mut uint8_t;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul((*hash).digest_size as libc::c_ulong) as usize,
    );
    h2 = fresh3.leak().as_mut_ptr() as *mut uint8_t;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (*hash).context_size as libc::c_ulong as usize,
    );
    state = fresh4.leak().as_mut_ptr() as *mut _;
    if !(key_size
        < ((*hash).digest_size as libc::c_ulong)
            .wrapping_add(salt_length)
            .wrapping_add(2 as libc::c_int as libc::c_ulong))
    {
        if !(__gmpz_sizeinbase(m, 2 as libc::c_int) > bits) {
            nettle_mpz_get_str_256(key_size, em, m);
            if !(*em
                .offset(
                    key_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != 0xbc as libc::c_int)
            {
                h = em
                    .offset(
                        key_size
                            .wrapping_sub((*hash).digest_size as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                if *em as libc::c_int
                    & !(pss_masks[(8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(key_size)
                        .wrapping_sub(bits) as usize] as libc::c_int) == 0 as libc::c_int
                {} else {
                    __assert_fail(
                        b"(*em & ~pss_masks[(8 * key_size - bits)]) == 0\0" as *const u8
                            as *const libc::c_char,
                        b"pss.c\0" as *const u8 as *const libc::c_char,
                        161 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 110],
                            &[libc::c_char; 110],
                        >(
                            b"int nettle_pss_verify_mgf1(const __mpz_struct *, size_t, const struct nettle_hash *, size_t, const uint8_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_4454: {
                    if *em as libc::c_int
                        & !(pss_masks[(8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(key_size)
                            .wrapping_sub(bits) as usize] as libc::c_int)
                        == 0 as libc::c_int
                    {} else {
                        __assert_fail(
                            b"(*em & ~pss_masks[(8 * key_size - bits)]) == 0\0"
                                as *const u8 as *const libc::c_char,
                            b"pss.c\0" as *const u8 as *const libc::c_char,
                            161 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 110],
                                &[libc::c_char; 110],
                            >(
                                b"int nettle_pss_verify_mgf1(const __mpz_struct *, size_t, const struct nettle_hash *, size_t, const uint8_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                ((*hash).init).unwrap()(state);
                ((*hash).update).unwrap()(state, (*hash).digest_size as size_t, h);
                db = em.offset(key_size as isize);
                nettle_pss_mgf1(
                    state,
                    hash,
                    key_size
                        .wrapping_sub((*hash).digest_size as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    db,
                );
                nettle_memxor(
                    db as *mut libc::c_void,
                    em as *const libc::c_void,
                    key_size
                        .wrapping_sub((*hash).digest_size as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                *db = (*db as libc::c_int
                    & pss_masks[(8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(key_size)
                        .wrapping_sub(bits) as usize] as libc::c_int) as uint8_t;
                j = 0 as libc::c_int as size_t;
                loop {
                    if !(j
                        < key_size
                            .wrapping_sub(salt_length)
                            .wrapping_sub((*hash).digest_size as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    {
                        current_block = 6057473163062296781;
                        break;
                    }
                    if *db.offset(j as isize) as libc::c_int != 0 as libc::c_int {
                        current_block = 494786892793898768;
                        break;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                match current_block {
                    494786892793898768 => {}
                    _ => {
                        if !(*db.offset(j as isize) as libc::c_int != 0x1 as libc::c_int)
                        {
                            salt = db
                                .offset(j as isize)
                                .offset(1 as libc::c_int as isize);
                            ((*hash).init).unwrap()(state);
                            ((*hash).update)
                                .unwrap()(
                                state,
                                ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
                                pss_pad.as_ptr(),
                            );
                            ((*hash).update)
                                .unwrap()(state, (*hash).digest_size as size_t, digest);
                            ((*hash).update).unwrap()(state, salt_length, salt);
                            ((*hash).digest)
                                .unwrap()(state, (*hash).digest_size as size_t, h2);
                            if !(memcmp(
                                h2 as *const libc::c_void,
                                h as *const libc::c_void,
                                (*hash).digest_size as libc::c_ulong,
                            ) != 0 as libc::c_int)
                            {
                                ret = 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
    return ret;
}
