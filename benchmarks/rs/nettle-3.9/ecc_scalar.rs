use ::libc;
extern "C" {
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_roinit_n(_: mpz_ptr, _: mp_srcptr, _: mp_size_t) -> mpz_srcptr;
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
    fn _nettle_mpz_set_n(r: *mut __mpz_struct, xp: *const mp_limb_t, xn: mp_size_t);
}
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
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
pub unsafe extern "C" fn nettle_ecc_scalar_init(
    mut s: *mut ecc_scalar,
    mut ecc: *const ecc_curve,
) {
    (*s).ecc = ecc;
    (*s).p = _nettle_gmp_alloc_limbs((*ecc).p.size as mp_size_t);
}
pub unsafe extern "C" fn nettle_ecc_scalar_clear(mut s: *mut ecc_scalar) {
    _nettle_gmp_free_limbs((*s).p, (*(*s).ecc).p.size as mp_size_t);
}
pub unsafe extern "C" fn nettle_ecc_scalar_set(
    mut s: *mut ecc_scalar,
    mut z: *const __mpz_struct,
) -> libc::c_int {
    let mut size: mp_size_t = (*(*s).ecc).p.size as mp_size_t;
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if (if (*z)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*z)._mp_size > 0 as libc::c_int) as libc::c_int
    }) <= 0 as libc::c_int
        || __gmpz_cmp(z, __gmpz_roinit_n(t.as_mut_ptr(), (*(*s).ecc).q.m, size))
            >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    _nettle_mpz_limbs_copy((*s).p, z, size);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_ecc_scalar_get(
    mut s: *const ecc_scalar,
    mut z: *mut __mpz_struct,
) {
    _nettle_mpz_set_n(z, (*s).p, (*(*s).ecc).p.size as mp_size_t);
}
