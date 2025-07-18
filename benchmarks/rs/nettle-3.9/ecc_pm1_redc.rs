use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_addmul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_submul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_cnd_add_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
}
pub type mp_limb_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
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
pub unsafe extern "C" fn _nettle_ecc_pm1_redc(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut xp: *mut mp_limb_t,
) {
    let mut i: libc::c_uint = 0;
    let mut hi: mp_limb_t = 0;
    let mut cy: mp_limb_t = 0;
    let mut shift: libc::c_uint = ((*m).size as libc::c_int
        * (64 as libc::c_int - 0 as libc::c_int) - (*m).bit_size as libc::c_int)
        as libc::c_uint;
    let mut k: mp_size_t = (*m).redc_size as mp_size_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*m).size as libc::c_uint {
        *xp
            .offset(
                i as isize,
            ) = __gmpn_submul_1(
            xp.offset(i as isize).offset(k as isize),
            (*m).redc_mpm1,
            (*m).size as libc::c_long - k,
            *xp.offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    hi = __gmpn_sub_n(
        xp,
        xp.offset((*m).size as libc::c_int as isize) as mp_srcptr,
        xp as mp_srcptr,
        (*m).size as mp_size_t,
    );
    cy = __gmpn_cnd_add_n(hi, rp, xp as mp_srcptr, (*m).m, (*m).size as mp_size_t);
    if cy == hi {} else {
        __assert_fail(
            b"cy == hi\0" as *const u8 as *const libc::c_char,
            b"ecc-pm1-redc.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void _nettle_ecc_pm1_redc(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3620: {
        if cy == hi {} else {
            __assert_fail(
                b"cy == hi\0" as *const u8 as *const libc::c_char,
                b"ecc-pm1-redc.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"void _nettle_ecc_pm1_redc(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if shift > 0 as libc::c_int as libc::c_uint {
        hi = *rp.offset(((*m).size as libc::c_int - 1 as libc::c_int) as isize)
            >> ((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                .wrapping_sub(shift);
        *rp
            .offset(
                ((*m).size as libc::c_int - 1 as libc::c_int) as isize,
            ) = (*rp.offset(((*m).size as libc::c_int - 1 as libc::c_int) as isize)
            & ((1 as libc::c_int as mp_limb_t)
                << ((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(shift))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_add(
                __gmpn_addmul_1(
                    rp,
                    (*m).B_shifted,
                    ((*m).size as libc::c_int - 1 as libc::c_int) as mp_size_t,
                    hi,
                ),
            );
    }
}
