use ::libc;
extern "C" {
    static _nettle_curve448: ecc_curve;
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_mpn_set_base256_le(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_ecc_mul_m(
        m: *const ecc_modulo,
        a24: mp_limb_t,
        bit_low: libc::c_uint,
        bit_high: libc::c_uint,
        qx: *mut mp_limb_t,
        n: *const uint8_t,
        px: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_mpn_get_base256_le(
        rp: *mut uint8_t,
        rn: size_t,
        xp: *const mp_limb_t,
        xn: mp_size_t,
    );
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_size_t = libc::c_long;
pub type mp_limb_t = libc::c_ulong;
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
pub unsafe extern "C" fn nettle_curve448_mul(
    mut q: *mut uint8_t,
    mut n: *const uint8_t,
    mut p: *const uint8_t,
) {
    let mut m: *const ecc_modulo = &_nettle_curve448.p;
    let mut itch: mp_size_t = 0;
    let mut x: *mut mp_limb_t = 0 as *mut mp_limb_t;
    itch = ((*m).size as libc::c_int + 8 as libc::c_int * (*m).size as libc::c_int)
        as mp_size_t;
    x = _nettle_gmp_alloc_limbs(itch);
    _nettle_mpn_set_base256_le(
        x,
        (*m).size as mp_size_t,
        p,
        56 as libc::c_int as size_t,
    );
    _nettle_ecc_mul_m(
        m,
        39081 as libc::c_int as mp_limb_t,
        2 as libc::c_int as libc::c_uint,
        446 as libc::c_int as libc::c_uint,
        x,
        n,
        x,
        x.offset((*m).size as libc::c_int as isize),
    );
    _nettle_mpn_get_base256_le(
        q,
        56 as libc::c_int as size_t,
        x,
        (*m).size as mp_size_t,
    );
    _nettle_gmp_free_limbs(x, itch);
}
