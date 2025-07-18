use ::libc;
extern "C" {
    fn __gmpz_limbs_write(_: mpz_ptr, _: mp_size_t) -> mp_ptr;
    fn __gmpz_limbs_finish(_: mpz_ptr, _: mp_size_t);
    fn nettle_ecc_ecdsa_sign(
        ecc: *const ecc_curve,
        zp: *const mp_limb_t,
        kp: *const mp_limb_t,
        length: size_t,
        digest: *const uint8_t,
        rp: *mut mp_limb_t,
        sp: *mut mp_limb_t,
        scratch: *mut mp_limb_t,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_ptr = *mut __mpz_struct;
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
pub struct ecc_scalar {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_signature {
    pub r: mpz_t,
    pub s: mpz_t,
}
pub unsafe extern "C" fn nettle_ecdsa_sign(
    mut key: *const ecc_scalar,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut digest_length: size_t,
    mut digest: *const uint8_t,
    mut signature: *mut dsa_signature,
) {
    let mut k: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut size: mp_limb_t = (*(*key).ecc).p.size as mp_limb_t;
    let mut rp: *mut mp_limb_t = __gmpz_limbs_write(
        ((*signature).r).as_mut_ptr(),
        size as mp_size_t,
    );
    let mut sp: *mut mp_limb_t = __gmpz_limbs_write(
        ((*signature).s).as_mut_ptr(),
        size as mp_size_t,
    );
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(
                size
                    .wrapping_add(
                        (11 as libc::c_int as libc::c_ulong).wrapping_mul(size),
                    ),
            ) as usize,
    );
    k = fresh0.leak().as_mut_ptr() as *mut mp_limb_t;
    loop {
        _nettle_ecc_mod_random(
            &(*(*key).ecc).q,
            k,
            random_ctx,
            random,
            k.offset(size as isize),
        );
        nettle_ecc_ecdsa_sign(
            (*key).ecc,
            (*key).p,
            k,
            digest_length,
            digest,
            rp,
            sp,
            k.offset(size as isize),
        );
        __gmpz_limbs_finish(((*signature).r).as_mut_ptr(), size as mp_size_t);
        __gmpz_limbs_finish(((*signature).s).as_mut_ptr(), size as mp_size_t);
        if !((if (*((*signature).r).as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*signature).r).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) == 0 as libc::c_int
            || (if (*((*signature).s).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*signature).s).as_mut_ptr())._mp_size > 0 as libc::c_int)
                    as libc::c_int
            }) == 0 as libc::c_int)
        {
            break;
        }
    };
}
