use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _nettle_mpn_set_base256_le(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_mpn_get_base256_le(
        rp: *mut uint8_t,
        rn: size_t,
        xp: *const mp_limb_t,
        xn: mp_size_t,
    );
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
    fn _nettle_ecc_mod_mul_canonical(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp: *mut mp_limb_t,
    );
    fn nettle_ecc_bit_size(ecc: *const ecc_curve) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_limb_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_point {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_scalar {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
#[inline]
unsafe extern "C" fn __gmpn_zero_p(
    mut __gmp_p: mp_srcptr,
    mut __gmp_n: mp_size_t,
) -> libc::c_int {
    loop {
        __gmp_n -= 1;
        if *__gmp_p.offset(__gmp_n as isize) != 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if !(__gmp_n != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_gostdsa_vko(
    mut priv_0: *const ecc_scalar,
    mut pub_0: *const ecc_point,
    mut ukm_length: size_t,
    mut ukm: *const uint8_t,
    mut out: *mut uint8_t,
) {
    let mut ecc: *const ecc_curve = (*priv_0).ecc;
    let mut bsize: libc::c_uint = (nettle_ecc_bit_size(ecc))
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    let mut size: mp_size_t = (*ecc).p.size as mp_size_t;
    let mut itch: mp_size_t = 4 as libc::c_int as libc::c_long * size
        + (*ecc).mul_itch as libc::c_long;
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    if itch
        < 5 as libc::c_int as libc::c_long * size + (*ecc).h_to_a_itch as libc::c_long
    {
        itch = 5 as libc::c_int as libc::c_long * size
            + (*ecc).h_to_a_itch as libc::c_long;
    }
    if (*pub_0).ecc == ecc {} else {
        __assert_fail(
            b"pub->ecc == ecc\0" as *const u8 as *const libc::c_char,
            b"gostdsa-vko.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void nettle_gostdsa_vko(const struct ecc_scalar *, const struct ecc_point *, size_t, const uint8_t *, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5378: {
        if (*pub_0).ecc == ecc {} else {
            __assert_fail(
                b"pub->ecc == ecc\0" as *const u8 as *const libc::c_char,
                b"gostdsa-vko.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void nettle_gostdsa_vko(const struct ecc_scalar *, const struct ecc_point *, size_t, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*priv_0).ecc == ecc {} else {
        __assert_fail(
            b"priv->ecc == ecc\0" as *const u8 as *const libc::c_char,
            b"gostdsa-vko.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void nettle_gostdsa_vko(const struct ecc_scalar *, const struct ecc_point *, size_t, const uint8_t *, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5335: {
        if (*priv_0).ecc == ecc {} else {
            __assert_fail(
                b"priv->ecc == ecc\0" as *const u8 as *const libc::c_char,
                b"gostdsa-vko.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void nettle_gostdsa_vko(const struct ecc_scalar *, const struct ecc_point *, size_t, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ukm_length <= bsize as libc::c_ulong {} else {
        __assert_fail(
            b"ukm_length <= bsize\0" as *const u8 as *const libc::c_char,
            b"gostdsa-vko.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void nettle_gostdsa_vko(const struct ecc_scalar *, const struct ecc_point *, size_t, const uint8_t *, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5290: {
        if ukm_length <= bsize as libc::c_ulong {} else {
            __assert_fail(
                b"ukm_length <= bsize\0" as *const u8 as *const libc::c_char,
                b"gostdsa-vko.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void nettle_gostdsa_vko(const struct ecc_scalar *, const struct ecc_point *, size_t, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    scratch = _nettle_gmp_alloc_limbs(itch);
    _nettle_mpn_set_base256_le(scratch, size, ukm, ukm_length);
    if __gmpn_zero_p(scratch as mp_srcptr, size) != 0 {
        *scratch.offset(0 as libc::c_int as isize) = 1 as libc::c_int as mp_limb_t;
    }
    _nettle_ecc_mod_mul_canonical(
        &(*ecc).q,
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize),
        (*priv_0).p,
        scratch,
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize),
    );
    ((*ecc).mul)
        .unwrap()(
        ecc,
        scratch,
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize),
        (*pub_0).p,
        scratch.offset((4 as libc::c_int as libc::c_long * size) as isize),
    );
    ((*ecc).h_to_a)
        .unwrap()(
        ecc,
        0 as libc::c_int,
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize),
        scratch,
        scratch.offset((5 as libc::c_int as libc::c_long * size) as isize),
    );
    _nettle_mpn_get_base256_le(
        out,
        bsize as size_t,
        scratch.offset((3 as libc::c_int as libc::c_long * size) as isize),
        size,
    );
    _nettle_mpn_get_base256_le(
        out.offset(bsize as isize),
        bsize as size_t,
        scratch.offset((4 as libc::c_int as libc::c_long * size) as isize),
        size,
    );
    _nettle_gmp_free_limbs(scratch, itch);
}
