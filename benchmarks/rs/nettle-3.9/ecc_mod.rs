use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_add_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_addmul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_cnd_add_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
    fn _nettle_sec_add_1(
        rp: *mut mp_limb_t,
        ap: *mut mp_limb_t,
        n: mp_size_t,
        b: mp_limb_t,
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
pub unsafe extern "C" fn _nettle_ecc_mod(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut xp: *mut mp_limb_t,
) {
    let mut hi: mp_limb_t = 0;
    let mut mn: mp_size_t = (*m).size as mp_size_t;
    let mut bn: mp_size_t = (*m).B_size as mp_size_t;
    let mut sn: mp_size_t = mn - bn;
    let mut rn: mp_size_t = 2 as libc::c_int as libc::c_long * mn;
    let mut i: mp_size_t = 0;
    let mut shift: libc::c_uint = 0;
    if bn < mn {} else {
        __assert_fail(
            b"bn < mn\0" as *const u8 as *const libc::c_char,
            b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4250: {
        if bn < mn {} else {
            __assert_fail(
                b"bn < mn\0" as *const u8 as *const libc::c_char,
                b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if *((*m).B).offset((bn - 1 as libc::c_int as libc::c_long) as isize)
        < (1 as libc::c_int as mp_limb_t)
            << 64 as libc::c_int - 0 as libc::c_int - 1 as libc::c_int
    {
        while rn > 2 as libc::c_int as libc::c_long * mn - bn {
            rn -= sn;
            i = 0 as libc::c_int as mp_size_t;
            while i <= sn {
                *xp
                    .offset(
                        (rn + i - 1 as libc::c_int as libc::c_long) as isize,
                    ) = __gmpn_addmul_1(
                    xp
                        .offset(rn as isize)
                        .offset(-(mn as isize))
                        .offset(-(1 as libc::c_int as isize))
                        .offset(i as isize),
                    (*m).B,
                    bn,
                    *xp.offset((rn + i - 1 as libc::c_int as libc::c_long) as isize),
                );
                i += 1;
                i;
            }
            *xp
                .offset(
                    (rn - 1 as libc::c_int as libc::c_long) as isize,
                ) = (*xp.offset((rn + sn - 1 as libc::c_int as libc::c_long) as isize))
                .wrapping_add(
                    __gmpn_add_n(
                        xp
                            .offset(rn as isize)
                            .offset(-(sn as isize))
                            .offset(-(1 as libc::c_int as isize)),
                        xp
                            .offset(rn as isize)
                            .offset(-(sn as isize))
                            .offset(-(1 as libc::c_int as isize)) as mp_srcptr,
                        xp.offset(rn as isize).offset(-(1 as libc::c_int as isize))
                            as mp_srcptr,
                        sn,
                    ),
                );
        }
    } else {
        while rn > 2 as libc::c_int as libc::c_long * mn - bn {
            rn -= sn;
            i = 0 as libc::c_int as mp_size_t;
            while i < sn {
                *xp
                    .offset(
                        (rn + i) as isize,
                    ) = __gmpn_addmul_1(
                    xp.offset(rn as isize).offset(-(mn as isize)).offset(i as isize),
                    (*m).B,
                    bn,
                    *xp.offset((rn + i) as isize),
                );
                i += 1;
                i;
            }
            hi = __gmpn_add_n(
                xp.offset(rn as isize).offset(-(sn as isize)),
                xp.offset(rn as isize).offset(-(sn as isize)) as mp_srcptr,
                xp.offset(rn as isize) as mp_srcptr,
                sn,
            );
            hi = __gmpn_cnd_add_n(
                hi,
                xp.offset(rn as isize).offset(-(mn as isize)),
                xp.offset(rn as isize).offset(-(mn as isize)) as mp_srcptr,
                (*m).B,
                mn,
            );
            if hi == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"hi == 0\0" as *const u8 as *const libc::c_char,
                    b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                    85 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_3930: {
                if hi == 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"hi == 0\0" as *const u8 as *const libc::c_char,
                        b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                        85 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 74],
                            &[libc::c_char; 74],
                        >(
                            b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    if rn > mn {} else {
        __assert_fail(
            b"rn > mn\0" as *const u8 as *const libc::c_char,
            b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3881: {
        if rn > mn {} else {
            __assert_fail(
                b"rn > mn\0" as *const u8 as *const libc::c_char,
                b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    rn -= mn;
    if rn <= sn {} else {
        __assert_fail(
            b"rn <= sn\0" as *const u8 as *const libc::c_char,
            b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3838: {
        if rn <= sn {} else {
            __assert_fail(
                b"rn <= sn\0" as *const u8 as *const libc::c_char,
                b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as mp_size_t;
    while i < rn {
        *xp
            .offset(
                (mn + i) as isize,
            ) = __gmpn_addmul_1(
            xp.offset(i as isize),
            (*m).B,
            bn,
            *xp.offset((mn + i) as isize),
        );
        i += 1;
        i;
    }
    hi = __gmpn_add_n(
        xp.offset(bn as isize),
        xp.offset(bn as isize) as mp_srcptr,
        xp.offset(mn as isize) as mp_srcptr,
        rn,
    );
    if rn < sn {
        hi = _nettle_sec_add_1(
            xp.offset(bn as isize).offset(rn as isize),
            xp.offset(bn as isize).offset(rn as isize),
            sn - rn,
            hi,
        );
    }
    shift = ((*m).size as libc::c_int * (64 as libc::c_int - 0 as libc::c_int)
        - (*m).bit_size as libc::c_int) as libc::c_uint;
    if shift > 0 as libc::c_int as libc::c_uint {
        hi = hi << shift
            | *xp.offset((mn - 1 as libc::c_int as libc::c_long) as isize)
                >> ((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(shift);
        *xp
            .offset(
                (mn - 1 as libc::c_int as libc::c_long) as isize,
            ) = (*xp.offset((mn - 1 as libc::c_int as libc::c_long) as isize)
            & ((1 as libc::c_int as mp_limb_t)
                << ((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(shift))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_add(
                __gmpn_addmul_1(
                    xp,
                    (*m).B_shifted,
                    mn - 1 as libc::c_int as libc::c_long,
                    hi,
                ),
            );
        if rp != xp {
            __gmpn_copyi(rp, xp as mp_srcptr, mn);
        }
    } else {
        hi = __gmpn_cnd_add_n(hi, rp, xp as mp_srcptr, (*m).B, mn);
        if hi == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3510: {
            if hi == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"hi == 0\0" as *const u8 as *const libc::c_char,
                    b"ecc-mod.c\0" as *const u8 as *const libc::c_char,
                    114 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"void _nettle_ecc_mod(const struct ecc_modulo *, mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    };
}
