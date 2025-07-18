use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_submul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_cnd_add_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
    fn _nettle_eddsa_compress_itch(ecc: *const ecc_curve) -> mp_size_t;
    fn _nettle_eddsa_compress(
        ecc: *const ecc_curve,
        r: *mut uint8_t,
        p: *mut mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_eddsa_hash(
        m: *const ecc_modulo,
        rp_0: *mut mp_limb_t,
        digest_size: size_t,
        digest: *const uint8_t,
    );
    fn _nettle_ecc_mod_add(
        m: *const ecc_modulo,
        rp_0: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
    );
    fn _nettle_ecc_mod_mul(
        m: *const ecc_modulo,
        rp_0: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp: *mut mp_limb_t,
    );
    fn _nettle_mpn_get_base256_le(
        rp_0: *mut uint8_t,
        rn: size_t,
        xp: *const mp_limb_t,
        xn: mp_size_t,
    );
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
pub type mp_limb_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_curve {
    pub p: ecc_modulo,
    pub q: ecc_modulo,
    pub use_redc: libc::c_ushort,
    pub pippenger_k: libc::c_ushort,
    pub pippenger_c: libc::c_ushort,
    pub add_hh_itch: libc::c_ushort,
    pub add_hhh_itch: libc::c_ushort,
    pub dup_itch: libc::c_ushort,
    pub mul_itch: libc::c_ushort,
    pub mul_g_itch: libc::c_ushort,
    pub h_to_a_itch: libc::c_ushort,
    pub add_hh: Option::<ecc_add_func>,
    pub add_hhh: Option::<ecc_add_func>,
    pub dup: Option::<ecc_dup_func>,
    pub mul: Option::<ecc_mul_func>,
    pub mul_g: Option::<ecc_mul_g_func>,
    pub h_to_a: Option::<ecc_h_to_a_func>,
    pub b: *const mp_limb_t,
    pub unit: *const mp_limb_t,
    pub pippenger_table: *const mp_limb_t,
}
pub type ecc_h_to_a_func = unsafe extern "C" fn(
    *const ecc_curve,
    libc::c_int,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mul_g_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mul_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_dup_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_add_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
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
pub type nettle_eddsa_dom_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_eddsa {
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
    pub dom: Option::<nettle_eddsa_dom_func>,
    pub low_mask: mp_limb_t,
    pub high_bit: mp_limb_t,
}
pub unsafe extern "C" fn _nettle_eddsa_sign_itch(
    mut ecc: *const ecc_curve,
) -> mp_size_t {
    if (*ecc).mul_g_itch as libc::c_long <= _nettle_eddsa_compress_itch(ecc) {} else {
        __assert_fail(
            b"ecc->mul_g_itch <= _eddsa_compress_itch (ecc)\0" as *const u8
                as *const libc::c_char,
            b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"mp_size_t _nettle_eddsa_sign_itch(const struct ecc_curve *)\0"))
                .as_ptr(),
        );
    }
    'c_3509: {
        if (*ecc).mul_g_itch as libc::c_long <= _nettle_eddsa_compress_itch(ecc)
        {} else {
            __assert_fail(
                b"ecc->mul_g_itch <= _eddsa_compress_itch (ecc)\0" as *const u8
                    as *const libc::c_char,
                b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"mp_size_t _nettle_eddsa_sign_itch(const struct ecc_curve *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (5 as libc::c_int * (*ecc).p.size as libc::c_int) as libc::c_long
        + _nettle_eddsa_compress_itch(ecc);
}
pub unsafe extern "C" fn _nettle_eddsa_sign(
    mut ecc: *const ecc_curve,
    mut eddsa: *const ecc_eddsa,
    mut ctx: *mut libc::c_void,
    mut pub_0: *const uint8_t,
    mut k1: *const uint8_t,
    mut k2: *const mp_limb_t,
    mut length: size_t,
    mut msg: *const uint8_t,
    mut signature: *mut uint8_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut size: mp_size_t = 0;
    let mut nbytes: size_t = 0;
    let mut q: mp_limb_t = 0;
    let mut cy: mp_limb_t = 0;
    size = (*ecc).p.size as mp_size_t;
    nbytes = (1 as libc::c_int + (*ecc).p.bit_size as libc::c_int / 8 as libc::c_int)
        as size_t;
    ((*eddsa).dom).unwrap()(ctx);
    ((*eddsa).update).unwrap()(ctx, nbytes, k1);
    ((*eddsa).update).unwrap()(ctx, length, msg);
    ((*eddsa).digest)
        .unwrap()(
        ctx,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nbytes),
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize)
            as *mut uint8_t,
    );
    _nettle_eddsa_hash(
        &(*ecc).q,
        scratch,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nbytes),
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize)
            as *mut uint8_t,
    );
    ((*ecc).mul_g)
        .unwrap()(
        ecc,
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        scratch,
        scratch.offset((5 as libc::c_int as libc::c_long * size) as isize),
    );
    _nettle_eddsa_compress(
        ecc,
        signature,
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        scratch.offset((5 as libc::c_int as libc::c_long * size) as isize),
    );
    ((*eddsa).dom).unwrap()(ctx);
    ((*eddsa).update).unwrap()(ctx, nbytes, signature);
    ((*eddsa).update).unwrap()(ctx, nbytes, pub_0);
    ((*eddsa).update).unwrap()(ctx, length, msg);
    ((*eddsa).digest)
        .unwrap()(
        ctx,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nbytes),
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize)
            as *mut uint8_t,
    );
    _nettle_eddsa_hash(
        &(*ecc).q,
        scratch.offset(size as isize),
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nbytes),
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize)
            as *mut uint8_t,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).q,
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        scratch.offset(size as isize),
        k2,
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
    );
    _nettle_ecc_mod_add(
        &(*ecc).q,
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        scratch,
    );
    if (*ecc).p.bit_size as libc::c_int == 255 as libc::c_int {
        let mut shift: libc::c_uint = (252 as libc::c_int
            - (64 as libc::c_int - 0 as libc::c_int)
                * ((*ecc).p.size as libc::c_int - 1 as libc::c_int)) as libc::c_uint;
        q = *scratch
            .offset((2 as libc::c_int as libc::c_long * size) as isize)
            .offset(((*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize) >> shift;
    } else {
        let mut shift_0: libc::c_uint = 0;
        if (*ecc).p.bit_size as libc::c_int == 448 as libc::c_int {} else {
            __assert_fail(
                b"ecc->p.bit_size == 448\0" as *const u8 as *const libc::c_char,
                b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 186],
                    &[libc::c_char; 186],
                >(
                    b"void _nettle_eddsa_sign(const struct ecc_curve *, const struct ecc_eddsa *, void *, const uint8_t *, const uint8_t *, const mp_limb_t *, size_t, const uint8_t *, uint8_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3845: {
            if (*ecc).p.bit_size as libc::c_int == 448 as libc::c_int {} else {
                __assert_fail(
                    b"ecc->p.bit_size == 448\0" as *const u8 as *const libc::c_char,
                    b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 186],
                        &[libc::c_char; 186],
                    >(
                        b"void _nettle_eddsa_sign(const struct ecc_curve *, const struct ecc_eddsa *, void *, const uint8_t *, const uint8_t *, const mp_limb_t *, size_t, const uint8_t *, uint8_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        shift_0 = (446 as libc::c_int
            - (64 as libc::c_int - 0 as libc::c_int)
                * ((*ecc).p.size as libc::c_int - 1 as libc::c_int)) as libc::c_uint;
        q = (*scratch
            .offset((2 as libc::c_int as libc::c_long * size) as isize)
            .offset(((*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize)
            >> shift_0)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    cy = __gmpn_submul_1(
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        (*ecc).q.m,
        (*ecc).p.size as mp_size_t,
        q,
    );
    if cy < 2 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cy < 2\0" as *const u8 as *const libc::c_char,
            b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 186],
                &[libc::c_char; 186],
            >(
                b"void _nettle_eddsa_sign(const struct ecc_curve *, const struct ecc_eddsa *, void *, const uint8_t *, const uint8_t *, const mp_limb_t *, size_t, const uint8_t *, uint8_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3720: {
        if cy < 2 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy < 2\0" as *const u8 as *const libc::c_char,
                b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 186],
                    &[libc::c_char; 186],
                >(
                    b"void _nettle_eddsa_sign(const struct ecc_curve *, const struct ecc_eddsa *, void *, const uint8_t *, const uint8_t *, const mp_limb_t *, size_t, const uint8_t *, uint8_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    cy = (cy as libc::c_ulong)
        .wrapping_sub(
            __gmpn_cnd_add_n(
                cy,
                scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
                scratch.offset((2 as libc::c_int as libc::c_long * size) as isize)
                    as mp_srcptr,
                (*ecc).q.m,
                (*ecc).p.size as mp_size_t,
            ),
        ) as mp_limb_t as mp_limb_t;
    if cy == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cy == 0\0" as *const u8 as *const libc::c_char,
            b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 186],
                &[libc::c_char; 186],
            >(
                b"void _nettle_eddsa_sign(const struct ecc_curve *, const struct ecc_eddsa *, void *, const uint8_t *, const uint8_t *, const mp_limb_t *, size_t, const uint8_t *, uint8_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3641: {
        if cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy == 0\0" as *const u8 as *const libc::c_char,
                b"eddsa-sign.c\0" as *const u8 as *const libc::c_char,
                121 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 186],
                    &[libc::c_char; 186],
                >(
                    b"void _nettle_eddsa_sign(const struct ecc_curve *, const struct ecc_eddsa *, void *, const uint8_t *, const uint8_t *, const mp_limb_t *, size_t, const uint8_t *, uint8_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_mpn_get_base256_le(
        signature.offset(nbytes as isize),
        nbytes,
        scratch.offset((2 as libc::c_int as libc::c_long * size) as isize),
        (*ecc).q.size as mp_size_t,
    );
}
