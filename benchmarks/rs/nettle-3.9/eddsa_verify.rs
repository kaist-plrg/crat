use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn _nettle_eddsa_decompress_itch(ecc: *const ecc_curve) -> mp_size_t;
    fn _nettle_eddsa_decompress(
        ecc: *const ecc_curve,
        p: *mut mp_limb_t,
        cp: *const uint8_t,
        scratch: *mut mp_limb_t,
    ) -> libc::c_int;
    fn _nettle_eddsa_hash(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        digest_size: size_t,
        digest: *const uint8_t,
    );
    fn _nettle_mpn_set_base256_le(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_ecc_mod_mul_canonical(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp: *mut mp_limb_t,
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
#[inline]
unsafe extern "C" fn __gmpn_cmp(
    mut __gmp_xp: mp_srcptr,
    mut __gmp_yp: mp_srcptr,
    mut __gmp_size: mp_size_t,
) -> libc::c_int {
    let mut __gmp_result: libc::c_int = 0;
    let mut __gmp_i: mp_size_t = 0;
    let mut __gmp_x: mp_limb_t = 0;
    let mut __gmp_y: mp_limb_t = 0;
    __gmp_result = 0 as libc::c_int;
    __gmp_i = __gmp_size;
    loop {
        __gmp_i -= 1;
        if !(__gmp_i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        __gmp_x = *__gmp_xp.offset(__gmp_i as isize);
        __gmp_y = *__gmp_yp.offset(__gmp_i as isize);
        if !(__gmp_x != __gmp_y) {
            continue;
        }
        __gmp_result = if __gmp_x > __gmp_y {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        break;
    }
    return __gmp_result;
}
unsafe extern "C" fn equal_h(
    mut p: *const ecc_modulo,
    mut x1: *const mp_limb_t,
    mut z1: *const mp_limb_t,
    mut x2: *const mp_limb_t,
    mut z2: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) -> libc::c_int {
    _nettle_ecc_mod_mul_canonical(p, scratch, x1, z2, scratch);
    _nettle_ecc_mod_mul_canonical(
        p,
        scratch.offset((*p).size as libc::c_int as isize),
        x2,
        z1,
        scratch.offset((*p).size as libc::c_int as isize),
    );
    return (__gmpn_cmp(
        scratch as mp_srcptr,
        scratch.offset((*p).size as libc::c_int as isize) as mp_srcptr,
        (*p).size as mp_size_t,
    ) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn _nettle_eddsa_verify_itch(
    mut ecc: *const ecc_curve,
) -> mp_size_t {
    if _nettle_eddsa_decompress_itch(ecc) <= (*ecc).mul_itch as libc::c_long {} else {
        __assert_fail(
            b"_eddsa_decompress_itch (ecc) <= ecc->mul_itch\0" as *const u8
                as *const libc::c_char,
            b"eddsa-verify.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"mp_size_t _nettle_eddsa_verify_itch(const struct ecc_curve *)\0"))
                .as_ptr(),
        );
    }
    'c_3522: {
        if _nettle_eddsa_decompress_itch(ecc) <= (*ecc).mul_itch as libc::c_long
        {} else {
            __assert_fail(
                b"_eddsa_decompress_itch (ecc) <= ecc->mul_itch\0" as *const u8
                    as *const libc::c_char,
                b"eddsa-verify.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"mp_size_t _nettle_eddsa_verify_itch(const struct ecc_curve *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (8 as libc::c_int * (*ecc).p.size as libc::c_int
        + (*ecc).mul_itch as libc::c_int) as mp_size_t;
}
pub unsafe extern "C" fn _nettle_eddsa_verify(
    mut ecc: *const ecc_curve,
    mut eddsa: *const ecc_eddsa,
    mut pub_0: *const uint8_t,
    mut A: *const mp_limb_t,
    mut ctx: *mut libc::c_void,
    mut length: size_t,
    mut msg: *const uint8_t,
    mut signature: *const uint8_t,
    mut scratch: *mut mp_limb_t,
) -> libc::c_int {
    let mut nbytes: size_t = 0;
    nbytes = (1 as libc::c_int + (*ecc).p.bit_size as libc::c_int / 8 as libc::c_int)
        as size_t;
    if _nettle_eddsa_decompress(
        ecc,
        scratch,
        signature,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    ) == 0
    {
        return 0 as libc::c_int;
    }
    _nettle_mpn_set_base256_le(
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).q.size as mp_size_t,
        signature.offset(nbytes as isize),
        nbytes,
    );
    if __gmpn_cmp(
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            as mp_srcptr,
        (*ecc).q.m,
        (*ecc).q.size as mp_size_t,
    ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    ((*eddsa).dom).unwrap()(ctx);
    ((*eddsa).update).unwrap()(ctx, nbytes, signature);
    ((*eddsa).update).unwrap()(ctx, nbytes, pub_0);
    ((*eddsa).update).unwrap()(ctx, length, msg);
    ((*eddsa).digest)
        .unwrap()(
        ctx,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nbytes),
        scratch.offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            as *mut uint8_t,
    );
    _nettle_eddsa_hash(
        &(*ecc).q,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nbytes),
        scratch.offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            as *mut uint8_t,
    );
    ((*ecc).mul)
        .unwrap()(
        ecc,
        scratch.offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        A,
        scratch.offset((8 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    ((*ecc).add_hh)
        .unwrap()(
        ecc,
        scratch.offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch,
        scratch.offset((8 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    __gmpn_copyi(
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            as mp_srcptr,
        (*ecc).q.size as mp_size_t,
    );
    ((*ecc).mul_g)
        .unwrap()(
        ecc,
        scratch,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((8 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    return (equal_h(
        &(*ecc).p,
        scratch.offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch
            .offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            .offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((8 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    ) != 0
        && equal_h(
            &(*ecc).p,
            scratch
                .offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
                .offset((*ecc).p.size as libc::c_int as isize),
            scratch
                .offset((5 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
                .offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
            scratch.offset((*ecc).p.size as libc::c_int as isize),
            scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
            scratch.offset((8 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        ) != 0) as libc::c_int;
}
