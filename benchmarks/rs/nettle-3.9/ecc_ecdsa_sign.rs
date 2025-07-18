use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _nettle_ecc_hash(
        m: *const ecc_modulo,
        hp_0: *mut mp_limb_t,
        length: size_t,
        digest: *const uint8_t,
    );
    fn _nettle_ecc_mod_mul_canonical(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
    fn _nettle_ecc_mod_add(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
    );
    fn _nettle_ecc_mod_mul(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
    fn _nettle_ecc_j_to_a(
        ecc: *const ecc_curve,
        op: libc::c_int,
        r: *mut mp_limb_t,
        p: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_ecc_mul_g(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        np: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub unsafe extern "C" fn nettle_ecc_ecdsa_sign_itch(
    mut ecc: *const ecc_curve,
) -> mp_size_t {
    if (*ecc).p.size as libc::c_int + (*ecc).p.invert_itch as libc::c_int
        <= 3 as libc::c_int * (*ecc).p.size as libc::c_int
            + 8 as libc::c_int * (*ecc).p.size as libc::c_int
    {} else {
        __assert_fail(
            b"ecc->p.size + ecc->p.invert_itch <= 3*ecc->p.size + ECC_MUL_G_ITCH (ecc->p.size)\0"
                as *const u8 as *const libc::c_char,
            b"ecc-ecdsa-sign.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"mp_size_t nettle_ecc_ecdsa_sign_itch(const struct ecc_curve *)\0"))
                .as_ptr(),
        );
    }
    'c_4703: {
        if (*ecc).p.size as libc::c_int + (*ecc).p.invert_itch as libc::c_int
            <= 3 as libc::c_int * (*ecc).p.size as libc::c_int
                + 8 as libc::c_int * (*ecc).p.size as libc::c_int
        {} else {
            __assert_fail(
                b"ecc->p.size + ecc->p.invert_itch <= 3*ecc->p.size + ECC_MUL_G_ITCH (ecc->p.size)\0"
                    as *const u8 as *const libc::c_char,
                b"ecc-ecdsa-sign.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"mp_size_t nettle_ecc_ecdsa_sign_itch(const struct ecc_curve *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (11 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t;
}
pub unsafe extern "C" fn nettle_ecc_ecdsa_sign(
    mut ecc: *const ecc_curve,
    mut zp: *const mp_limb_t,
    mut kp: *const mp_limb_t,
    mut length: size_t,
    mut digest: *const uint8_t,
    mut rp: *mut mp_limb_t,
    mut sp: *mut mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    _nettle_ecc_mul_g(
        ecc,
        scratch,
        kp,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_j_to_a(
        ecc,
        2 as libc::c_int,
        rp,
        scratch,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    ((*ecc).q.invert)
        .unwrap()(
        &(*ecc).q,
        scratch,
        kp,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_hash(
        &(*ecc).q,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        length,
        digest,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).q,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        zp,
        rp,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_add(
        &(*ecc).q,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_mul_canonical(
        &(*ecc).q,
        sp,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
}
