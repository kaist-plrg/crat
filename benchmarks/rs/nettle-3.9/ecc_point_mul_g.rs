use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
}
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
pub unsafe extern "C" fn nettle_ecc_point_mul_g(
    mut r: *mut ecc_point,
    mut n: *const ecc_scalar,
) {
    let mut ecc: *const ecc_curve = (*r).ecc;
    let mut size: mp_limb_t = (*ecc).p.size as mp_limb_t;
    let mut itch: mp_size_t = (3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(size)
        .wrapping_add((*ecc).mul_g_itch as libc::c_ulong) as mp_size_t;
    let mut scratch: *mut mp_limb_t = _nettle_gmp_alloc_limbs(itch);
    if (*n).ecc == ecc {} else {
        __assert_fail(
            b"n->ecc == ecc\0" as *const u8 as *const libc::c_char,
            b"ecc-point-mul-g.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_ecc_point_mul_g(struct ecc_point *, const struct ecc_scalar *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3635: {
        if (*n).ecc == ecc {} else {
            __assert_fail(
                b"n->ecc == ecc\0" as *const u8 as *const libc::c_char,
                b"ecc-point-mul-g.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_ecc_point_mul_g(struct ecc_point *, const struct ecc_scalar *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ecc).h_to_a_itch as libc::c_int <= (*ecc).mul_g_itch as libc::c_int {} else {
        __assert_fail(
            b"ecc->h_to_a_itch <= ecc->mul_g_itch\0" as *const u8 as *const libc::c_char,
            b"ecc-point-mul-g.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_ecc_point_mul_g(struct ecc_point *, const struct ecc_scalar *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3580: {
        if (*ecc).h_to_a_itch as libc::c_int <= (*ecc).mul_g_itch as libc::c_int
        {} else {
            __assert_fail(
                b"ecc->h_to_a_itch <= ecc->mul_g_itch\0" as *const u8
                    as *const libc::c_char,
                b"ecc-point-mul-g.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_ecc_point_mul_g(struct ecc_point *, const struct ecc_scalar *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    ((*ecc).mul_g)
        .unwrap()(
        ecc,
        scratch,
        (*n).p,
        scratch.offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(size) as isize),
    );
    ((*ecc).h_to_a)
        .unwrap()(
        ecc,
        0 as libc::c_int,
        (*r).p,
        scratch,
        scratch.offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(size) as isize),
    );
    _nettle_gmp_free_limbs(scratch, itch);
}
