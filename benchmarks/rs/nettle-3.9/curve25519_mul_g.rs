use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static _nettle_curve25519: ecc_curve;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
    fn _nettle_mpn_get_base256_le(
        rp: *mut uint8_t,
        rn: size_t,
        xp: *const mp_limb_t,
        xn: mp_size_t,
    );
    fn _nettle_ecc_mul_g_eh(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        np: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_curve25519_eh_to_x(
        xp: *mut mp_limb_t,
        p: *const mp_limb_t,
        scratch: *mut mp_limb_t,
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
pub unsafe extern "C" fn nettle_curve25519_mul_g(
    mut r: *mut uint8_t,
    mut n: *const uint8_t,
) {
    let mut ecc: *const ecc_curve = &_nettle_curve25519;
    let mut t: [uint8_t; 32] = [0; 32];
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut itch: mp_size_t = 0;
    memcpy(
        t.as_mut_ptr() as *mut libc::c_void,
        n as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
    );
    t[0 as libc::c_int
        as usize] = (t[0 as libc::c_int as usize] as libc::c_int & !(7 as libc::c_int))
        as uint8_t;
    t[(32 as libc::c_int - 1 as libc::c_int)
        as usize] = (t[(32 as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
        & 0x3f as libc::c_int | 0x40 as libc::c_int) as uint8_t;
    itch = (4 as libc::c_int * (*ecc).p.size as libc::c_int
        + (*ecc).mul_g_itch as libc::c_int) as mp_size_t;
    scratch = _nettle_gmp_alloc_limbs(itch);
    _nettle_mpn_set_base256_le(
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).p.size as mp_size_t,
        t.as_mut_ptr(),
        32 as libc::c_int as size_t,
    );
    _nettle_ecc_mul_g_eh(
        ecc,
        scratch,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_curve25519_eh_to_x(
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch,
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_mpn_get_base256_le(
        r,
        32 as libc::c_int as size_t,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).p.size as mp_size_t,
    );
    _nettle_gmp_free_limbs(scratch, itch);
}
