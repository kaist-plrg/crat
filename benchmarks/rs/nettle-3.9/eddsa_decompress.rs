use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn _nettle_ecc_mod_sub(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
    );
    fn _nettle_ecc_mod_sqr(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
    fn _nettle_mpn_set_base256_le(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp_0: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_cnd_copy(
        cnd: libc::c_int,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        n: mp_size_t,
    );
    fn _nettle_ecc_mod_mul(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub unsafe extern "C" fn _nettle_eddsa_decompress_itch(
    mut ecc: *const ecc_curve,
) -> mp_size_t {
    return (4 as libc::c_int * (*ecc).p.size as libc::c_int
        + (*ecc).p.sqrt_ratio_itch as libc::c_int) as mp_size_t;
}
pub unsafe extern "C" fn _nettle_eddsa_decompress(
    mut ecc: *const ecc_curve,
    mut p: *mut mp_limb_t,
    mut cp: *const uint8_t,
    mut scratch: *mut mp_limb_t,
) -> libc::c_int {
    let mut sign: mp_limb_t = 0;
    let mut cy: mp_limb_t = 0;
    let mut nlimbs: mp_size_t = 0;
    let mut nbytes: size_t = 0;
    let mut res: libc::c_int = 0;
    nbytes = (1 as libc::c_int + (*ecc).p.bit_size as libc::c_int / 8 as libc::c_int)
        as size_t;
    sign = (*cp.offset(nbytes.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int >> 7 as libc::c_int) as mp_limb_t;
    nlimbs = nbytes
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        as mp_size_t;
    if nlimbs <= ((*ecc).p.size as libc::c_int + 1 as libc::c_int) as libc::c_long
    {} else {
        __assert_fail(
            b"nlimbs <= ecc->p.size + 1\0" as *const u8 as *const libc::c_char,
            b"eddsa-decompress.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"int _nettle_eddsa_decompress(const struct ecc_curve *, mp_limb_t *, const uint8_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4031: {
        if nlimbs <= ((*ecc).p.size as libc::c_int + 1 as libc::c_int) as libc::c_long
        {} else {
            __assert_fail(
                b"nlimbs <= ecc->p.size + 1\0" as *const u8 as *const libc::c_char,
                b"eddsa-decompress.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"int _nettle_eddsa_decompress(const struct ecc_curve *, mp_limb_t *, const uint8_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_mpn_set_base256_le(scratch, nlimbs, cp, nbytes);
    let ref mut fresh0 = *scratch
        .offset((nlimbs - 1 as libc::c_int as libc::c_long) as isize);
    *fresh0
        &= ((1 as libc::c_int as mp_limb_t)
            << nbytes
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_rem((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    __gmpn_copyi(
        p.offset((*ecc).p.size as libc::c_int as isize),
        scratch as mp_srcptr,
        (*ecc).p.size as mp_size_t,
    );
    if nlimbs > (*ecc).p.size as libc::c_long {
        res = (*scratch.offset((nlimbs - 1 as libc::c_int as libc::c_long) as isize)
            == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    } else {
        res = 1 as libc::c_int;
    }
    res = (res as libc::c_ulong
        & __gmpn_sub_n(
            scratch,
            scratch as mp_srcptr,
            (*ecc).p.m,
            (*ecc).p.size as mp_size_t,
        )) as libc::c_int;
    _nettle_ecc_mod_sqr(
        &(*ecc).p,
        scratch,
        p.offset((*ecc).p.size as libc::c_int as isize),
        scratch,
    );
    _nettle_ecc_mod_mul(
        &(*ecc).p,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch,
        (*ecc).b,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
    );
    _nettle_ecc_mod_sub(
        &(*ecc).p,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        (*ecc).unit,
    );
    if (*ecc).p.bit_size as libc::c_int == 255 as libc::c_int {
        _nettle_ecc_mod_sub(&(*ecc).p, scratch, (*ecc).unit, scratch);
    } else {
        _nettle_ecc_mod_sub(&(*ecc).p, scratch, scratch, (*ecc).unit);
    }
    res
        &= ((*ecc).p.sqrt_ratio)
            .unwrap()(
            &(*ecc).p,
            scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
            scratch,
            scratch.offset((*ecc).p.size as libc::c_int as isize),
            scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        );
    cy = __gmpn_sub_n(
        p,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            as mp_srcptr,
        (*ecc).p.m,
        (*ecc).p.size as mp_size_t,
    );
    _nettle_cnd_copy(
        cy as libc::c_int,
        p,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).p.size as mp_size_t,
    );
    sign ^= *p.offset(0 as libc::c_int as isize) & 1 as libc::c_int as libc::c_ulong;
    __gmpn_sub_n(
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).p.m,
        p as mp_srcptr,
        (*ecc).p.size as mp_size_t,
    );
    _nettle_cnd_copy(
        sign as libc::c_int,
        p,
        scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).p.size as mp_size_t,
    );
    res = (res as libc::c_ulong
        & __gmpn_sub_n(
            scratch.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
            p as mp_srcptr,
            (*ecc).p.m,
            (*ecc).p.size as mp_size_t,
        )) as libc::c_int;
    return res;
}
