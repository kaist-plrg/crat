use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_addmul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_cnd_add_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
    fn _nettle_mpn_set_base256_le(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_cnd_copy(
        cnd: libc::c_int,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        n: mp_size_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_limb_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_modulo {
    pub bit_size: libc::c_ushort,
    pub size: libc::c_ushort,
    pub B_size: libc::c_ushort,
    pub redc_size: libc::c_ushort,
    pub invert_itch: libc::c_ushort,
    pub sqrt_itch: libc::c_ushort,
    pub sqrt_ratio_itch: libc::c_ushort,
    pub m: *const mp_limb_t,
    pub B: *const mp_limb_t,
    pub B_shifted: *const mp_limb_t,
    pub Bm2m: *const mp_limb_t,
    pub redc_mpm1: *const mp_limb_t,
    pub mp1h: *const mp_limb_t,
    pub mod_0: Option::<ecc_mod_func>,
    pub reduce: Option::<ecc_mod_func>,
    pub invert: Option::<ecc_mod_inv_func>,
    pub sqrt: Option::<ecc_mod_sqrt_func>,
    pub sqrt_ratio: Option::<ecc_mod_sqrt_ratio_func>,
}
pub type ecc_mod_sqrt_ratio_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> libc::c_int;
pub type ecc_mod_sqrt_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> libc::c_int;
pub type ecc_mod_inv_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mod_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub unsafe extern "C" fn _nettle_eddsa_hash(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut digest_size: size_t,
    mut digest: *const uint8_t,
) {
    let mut nlimbs: mp_size_t = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(digest_size)
        .wrapping_add((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        as mp_size_t;
    let mut cy: mp_limb_t = 0;
    _nettle_mpn_set_base256_le(rp, nlimbs, digest, digest_size);
    if nlimbs > (2 as libc::c_int * (*m).size as libc::c_int) as libc::c_long {
        let mut hi: mp_limb_t = *rp
            .offset((2 as libc::c_int * (*m).size as libc::c_int) as isize);
        if nlimbs
            == (2 as libc::c_int * (*m).size as libc::c_int + 1 as libc::c_int)
                as libc::c_long
        {} else {
            __assert_fail(
                b"nlimbs == 2*m->size + 1\0" as *const u8 as *const libc::c_char,
                b"eddsa-hash.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void _nettle_eddsa_hash(const struct ecc_modulo *, mp_limb_t *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3728: {
            if nlimbs
                == (2 as libc::c_int * (*m).size as libc::c_int + 1 as libc::c_int)
                    as libc::c_long
            {} else {
                __assert_fail(
                    b"nlimbs == 2*m->size + 1\0" as *const u8 as *const libc::c_char,
                    b"eddsa-hash.c\0" as *const u8 as *const libc::c_char,
                    71 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 89],
                        &[libc::c_char; 89],
                    >(
                        b"void _nettle_eddsa_hash(const struct ecc_modulo *, mp_limb_t *, size_t, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        hi = __gmpn_addmul_1(
            rp.offset((*m).size as libc::c_int as isize),
            (*m).B,
            (*m).size as mp_size_t,
            hi,
        );
        if hi <= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi <= 1\0" as *const u8 as *const libc::c_char,
                b"eddsa-hash.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void _nettle_eddsa_hash(const struct ecc_modulo *, mp_limb_t *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3665: {
            if hi <= 1 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"hi <= 1\0" as *const u8 as *const libc::c_char,
                    b"eddsa-hash.c\0" as *const u8 as *const libc::c_char,
                    74 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 89],
                        &[libc::c_char; 89],
                    >(
                        b"void _nettle_eddsa_hash(const struct ecc_modulo *, mp_limb_t *, size_t, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        hi = __gmpn_cnd_add_n(
            hi,
            rp.offset((*m).size as libc::c_int as isize),
            rp.offset((*m).size as libc::c_int as isize) as mp_srcptr,
            (*m).B,
            (*m).size as mp_size_t,
        );
        if hi == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi == 0\0" as *const u8 as *const libc::c_char,
                b"eddsa-hash.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void _nettle_eddsa_hash(const struct ecc_modulo *, mp_limb_t *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3576: {
            if hi == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"hi == 0\0" as *const u8 as *const libc::c_char,
                    b"eddsa-hash.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 89],
                        &[libc::c_char; 89],
                    >(
                        b"void _nettle_eddsa_hash(const struct ecc_modulo *, mp_limb_t *, size_t, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    ((*m).mod_0).unwrap()(m, rp.offset((*m).size as libc::c_int as isize), rp);
    cy = __gmpn_sub_n(
        rp,
        rp.offset((*m).size as libc::c_int as isize) as mp_srcptr,
        (*m).m,
        (*m).size as mp_size_t,
    );
    _nettle_cnd_copy(
        cy as libc::c_int,
        rp,
        rp.offset((*m).size as libc::c_int as isize),
        (*m).size as mp_size_t,
    );
}
