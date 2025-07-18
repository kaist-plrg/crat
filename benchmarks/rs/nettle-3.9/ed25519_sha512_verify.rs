use ::libc;
extern "C" {
    static _nettle_ed25519_sha512: ecc_eddsa;
    fn _nettle_eddsa_decompress(
        ecc: *const ecc_curve,
        p: *mut mp_limb_t,
        cp: *const uint8_t,
        scratch: *mut mp_limb_t,
    ) -> libc::c_int;
    fn _nettle_eddsa_verify_itch(ecc: *const ecc_curve) -> mp_size_t;
    fn _nettle_eddsa_verify(
        ecc: *const ecc_curve,
        eddsa: *const ecc_eddsa,
        pub_0: *const uint8_t,
        A_0: *const mp_limb_t,
        ctx: *mut libc::c_void,
        length: size_t,
        msg: *const uint8_t,
        signature: *const uint8_t,
        scratch: *mut mp_limb_t,
    ) -> libc::c_int;
    static _nettle_curve25519: ecc_curve;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn nettle_sha512_init(ctx: *mut sha512_ctx);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_eddsa {
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
    pub dom: Option::<nettle_eddsa_dom_func>,
    pub low_mask: mp_limb_t,
    pub high_bit: mp_limb_t,
}
pub type nettle_eddsa_dom_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub unsafe extern "C" fn nettle_ed25519_sha512_verify(
    mut pub_0: *const uint8_t,
    mut length: size_t,
    mut msg: *const uint8_t,
    mut signature: *const uint8_t,
) -> libc::c_int {
    let mut ecc: *const ecc_curve = &_nettle_curve25519;
    let mut itch: mp_size_t = (3 as libc::c_int * (*ecc).p.size as libc::c_int)
        as libc::c_long + _nettle_eddsa_verify_itch(ecc);
    let mut scratch: *mut mp_limb_t = _nettle_gmp_alloc_limbs(itch);
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        count_low: 0,
        count_high: 0,
        index: 0,
        block: [0; 128],
    };
    let mut res: libc::c_int = 0;
    nettle_sha512_init(&mut ctx);
    res = (_nettle_eddsa_decompress(
        ecc,
        scratch,
        pub_0,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    ) != 0
        && _nettle_eddsa_verify(
            ecc,
            &_nettle_ed25519_sha512,
            pub_0,
            scratch,
            &mut ctx as *mut sha512_ctx as *mut libc::c_void,
            length,
            msg,
            signature,
            scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        ) != 0) as libc::c_int;
    _nettle_gmp_free_limbs(scratch, itch);
    return res;
}
