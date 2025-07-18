use ::libc;
extern "C" {
    fn __gmpn_rshift(
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: libc::c_uint,
    ) -> mp_limb_t;
    fn _nettle_mpn_set_base256(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_mpn_set_base256_le(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
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
pub unsafe extern "C" fn _nettle_ecc_hash(
    mut m: *const ecc_modulo,
    mut hp: *mut mp_limb_t,
    mut length: size_t,
    mut digest: *const uint8_t,
) {
    if length
        > ((*m).bit_size as size_t)
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            .wrapping_div(8 as libc::c_int as libc::c_ulong)
    {
        length = (((*m).bit_size as libc::c_int + 7 as libc::c_int) / 8 as libc::c_int)
            as size_t;
    }
    _nettle_mpn_set_base256(
        hp,
        ((*m).size as libc::c_int + 1 as libc::c_int) as mp_size_t,
        digest,
        length,
    );
    if (8 as libc::c_int as libc::c_ulong).wrapping_mul(length)
        > (*m).bit_size as libc::c_ulong
    {
        __gmpn_rshift(
            hp,
            hp as mp_srcptr,
            ((*m).size as libc::c_int + 1 as libc::c_int) as mp_size_t,
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(length)
                .wrapping_sub((*m).bit_size as libc::c_ulong) as libc::c_uint,
        );
    }
}
pub unsafe extern "C" fn _nettle_gost_hash(
    mut m: *const ecc_modulo,
    mut hp: *mut mp_limb_t,
    mut length: size_t,
    mut digest: *const uint8_t,
) {
    if length
        > ((*m).bit_size as size_t)
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            .wrapping_div(8 as libc::c_int as libc::c_ulong)
    {
        length = (((*m).bit_size as libc::c_int + 7 as libc::c_int) / 8 as libc::c_int)
            as size_t;
    }
    _nettle_mpn_set_base256_le(
        hp,
        ((*m).size as libc::c_int + 1 as libc::c_int) as mp_size_t,
        digest,
        length,
    );
}
