use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _nettle_ecc_mod_random(
        m: *const ecc_modulo,
        xp: *mut mp_limb_t,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        scratch: *mut mp_limb_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_random_func = unsafe extern "C" fn(
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
pub unsafe extern "C" fn nettle_ecdsa_generate_keypair(
    mut pub_0: *mut ecc_point,
    mut key: *mut ecc_scalar,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
) {
    let mut p: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut ecc: *const ecc_curve = (*pub_0).ecc;
    let mut itch: mp_size_t = (3 as libc::c_int * (*ecc).p.size as libc::c_int
        + (*ecc).mul_g_itch as libc::c_int) as mp_size_t;
    if (*key).ecc == ecc {} else {
        __assert_fail(
            b"key->ecc == ecc\0" as *const u8 as *const libc::c_char,
            b"ecdsa-keygen.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 106],
                &[libc::c_char; 106],
            >(
                b"void nettle_ecdsa_generate_keypair(struct ecc_point *, struct ecc_scalar *, void *, nettle_random_func *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4871: {
        if (*key).ecc == ecc {} else {
            __assert_fail(
                b"key->ecc == ecc\0" as *const u8 as *const libc::c_char,
                b"ecdsa-keygen.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 106],
                    &[libc::c_char; 106],
                >(
                    b"void nettle_ecdsa_generate_keypair(struct ecc_point *, struct ecc_scalar *, void *, nettle_random_func *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ecc).h_to_a_itch as libc::c_int <= (*ecc).mul_g_itch as libc::c_int {} else {
        __assert_fail(
            b"ecc->h_to_a_itch <= ecc->mul_g_itch\0" as *const u8 as *const libc::c_char,
            b"ecdsa-keygen.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 106],
                &[libc::c_char; 106],
            >(
                b"void nettle_ecdsa_generate_keypair(struct ecc_point *, struct ecc_scalar *, void *, nettle_random_func *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4816: {
        if (*ecc).h_to_a_itch as libc::c_int <= (*ecc).mul_g_itch as libc::c_int
        {} else {
            __assert_fail(
                b"ecc->h_to_a_itch <= ecc->mul_g_itch\0" as *const u8
                    as *const libc::c_char,
                b"ecdsa-keygen.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 106],
                    &[libc::c_char; 106],
                >(
                    b"void nettle_ecdsa_generate_keypair(struct ecc_point *, struct ecc_scalar *, void *, nettle_random_func *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(itch as libc::c_ulong) as usize,
    );
    p = fresh0.leak().as_mut_ptr() as *mut mp_limb_t;
    _nettle_ecc_mod_random(&(*ecc).q, (*key).p, random_ctx, random, p);
    ((*ecc).mul_g)
        .unwrap()(
        ecc,
        p,
        (*key).p,
        p.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    ((*ecc).h_to_a)
        .unwrap()(
        ecc,
        0 as libc::c_int,
        (*pub_0).p,
        p,
        p.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
}
