use ::libc;
extern "C" {
    fn _nettle_ecc_mod_add(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
    );
    fn _nettle_ecc_mod_sub(
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
        tp: *mut mp_limb_t,
    );
    fn _nettle_ecc_mod_sqr(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        tp: *mut mp_limb_t,
    );
}
pub type mp_limb_t = libc::c_ulong;
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
pub unsafe extern "C" fn _nettle_ecc_add_thh(
    mut ecc: *const ecc_curve,
    mut r: *mut mp_limb_t,
    mut p: *const mp_limb_t,
    mut q: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        p,
        q,
        scratch,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        p.offset((*ecc).p.size as libc::c_int as isize),
        q.offset((*ecc).p.size as libc::c_int as isize),
        scratch,
    );
    _nettle_ecc_mod_add(
        &(*ecc).p,
        r,
        p,
        p.offset((*ecc).p.size as libc::c_int as isize),
    );
    _nettle_ecc_mod_add(
        &(*ecc).p,
        r.offset((*ecc).p.size as libc::c_int as isize),
        q,
        q.offset((*ecc).p.size as libc::c_int as isize),
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch,
        r,
        r.offset((*ecc).p.size as libc::c_int as isize),
        scratch,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).b,
        r,
    );
    _nettle_ecc_mod_add(
        &(*ecc).p,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_sub(
        &(*ecc).p,
        scratch,
        scratch,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        p.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        q.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch,
        scratch,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r,
    );
    _nettle_ecc_mod_sqr(
        &(*ecc).p,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r,
    );
    _nettle_ecc_mod_add(
        &(*ecc).p,
        r,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_sub(
        &(*ecc).p,
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        r.offset((*ecc).p.size as libc::c_int as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r.offset((*ecc).p.size as libc::c_int as isize),
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        r.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        r,
        scratch.offset((1 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        r,
        r,
        scratch,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
}
