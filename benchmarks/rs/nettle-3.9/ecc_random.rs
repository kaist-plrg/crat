use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn _nettle_sec_zero_p(ap: *const mp_limb_t, n: mp_size_t) -> libc::c_int;
    fn _nettle_mpn_set_base256(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
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
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
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
pub struct ecc_scalar {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
unsafe extern "C" fn ecdsa_in_range(
    mut m: *const ecc_modulo,
    mut xp: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) -> libc::c_int {
    return (_nettle_sec_zero_p(xp, (*m).size as mp_size_t) == 0) as libc::c_int
        & (__gmpn_sub_n(scratch, xp, (*m).m, (*m).size as mp_size_t)
            != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn _nettle_ecc_mod_random(
    mut m: *const ecc_modulo,
    mut xp: *mut mp_limb_t,
    mut ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut scratch: *mut mp_limb_t,
) {
    let mut buf: *mut uint8_t = scratch as *mut uint8_t;
    let mut nbytes: libc::c_uint = (((*m).bit_size as libc::c_int + 7 as libc::c_int)
        / 8 as libc::c_int) as libc::c_uint;
    if nbytes as libc::c_ulong
        <= ((*m).size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"nbytes <= m->size * sizeof (mp_limb_t)\0" as *const u8
                as *const libc::c_char,
            b"ecc-random.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"void _nettle_ecc_mod_random(const struct ecc_modulo *, mp_limb_t *, void *, nettle_random_func *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3635: {
        if nbytes as libc::c_ulong
            <= ((*m).size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        {} else {
            __assert_fail(
                b"nbytes <= m->size * sizeof (mp_limb_t)\0" as *const u8
                    as *const libc::c_char,
                b"ecc-random.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"void _nettle_ecc_mod_random(const struct ecc_modulo *, mp_limb_t *, void *, nettle_random_func *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    loop {
        random.unwrap()(ctx, nbytes as size_t, buf);
        let ref mut fresh0 = *buf.offset(0 as libc::c_int as isize);
        *fresh0 = (*fresh0 as libc::c_int
            & 0xff as libc::c_int
                >> nbytes
                    .wrapping_mul(8 as libc::c_int as libc::c_uint)
                    .wrapping_sub((*m).bit_size as libc::c_uint)) as uint8_t;
        _nettle_mpn_set_base256(xp, (*m).size as mp_size_t, buf, nbytes as size_t);
        if !(ecdsa_in_range(m, xp, scratch) == 0) {
            break;
        }
    };
}
pub unsafe extern "C" fn nettle_ecc_scalar_random(
    mut x: *mut ecc_scalar,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
) {
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul((*(*x).ecc).q.size as libc::c_ulong) as usize,
    );
    scratch = fresh1.leak().as_mut_ptr() as *mut mp_limb_t;
    _nettle_ecc_mod_random(&(*(*x).ecc).q, (*x).p, random_ctx, random, scratch);
}
