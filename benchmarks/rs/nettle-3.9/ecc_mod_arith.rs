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
    fn __gmpn_mul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_mul_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_sqr(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_submul_1(_: mp_ptr, _: mp_srcptr, _: mp_size_t, _: mp_limb_t) -> mp_limb_t;
    fn __gmpn_cnd_add_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
    fn __gmpn_cnd_sub_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
    fn _nettle_cnd_copy(
        cnd: libc::c_int,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        n: mp_size_t,
    );
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
pub unsafe extern "C" fn _nettle_ecc_mod_zero_p(
    mut m: *const ecc_modulo,
    mut xp_in: *const mp_limb_t,
) -> libc::c_int {
    let mut is_non_zero: mp_limb_t = 0;
    let mut is_not_p: mp_limb_t = 0;
    let mut xp: *const mp_limb_t = 0 as *const mp_limb_t;
    let mut i: mp_size_t = 0;
    xp = xp_in;
    i = 0 as libc::c_int as mp_size_t;
    ::std::ptr::write_volatile(
        &mut is_not_p as *mut mp_limb_t,
        0 as libc::c_int as mp_limb_t,
    );
    ::std::ptr::write_volatile(
        &mut is_non_zero as *mut mp_limb_t,
        ::std::ptr::read_volatile::<mp_limb_t>(&is_not_p as *const mp_limb_t),
    );
    while i < (*m).size as libc::c_long {
        ::std::ptr::write_volatile(
            &mut is_non_zero as *mut mp_limb_t,
            (::std::ptr::read_volatile::<mp_limb_t>(&is_non_zero as *const mp_limb_t)
                as libc::c_ulong | *xp.offset(i as isize)) as mp_limb_t as mp_limb_t,
        );
        ::std::ptr::write_volatile(
            &mut is_not_p as *mut mp_limb_t,
            (::std::ptr::read_volatile::<mp_limb_t>(&is_not_p as *const mp_limb_t)
                as libc::c_ulong | *xp.offset(i as isize) ^ *((*m).m).offset(i as isize))
                as mp_limb_t as mp_limb_t,
        );
        i += 1;
        i;
    }
    return (is_non_zero == 0 as libc::c_int as libc::c_ulong) as libc::c_int
        | (is_not_p == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn _nettle_ecc_mod_equal_p(
    mut m: *const ecc_modulo,
    mut a: *const mp_limb_t,
    mut ref_0: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) -> libc::c_int {
    let mut cy: mp_limb_t = 0;
    cy = __gmpn_sub_n(scratch, a, ref_0, (*m).size as mp_size_t);
    return (cy == 0 as libc::c_int as libc::c_ulong) as libc::c_int
        & _nettle_ecc_mod_zero_p(m, scratch);
}
pub unsafe extern "C" fn _nettle_ecc_mod_add(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut bp: *const mp_limb_t,
) {
    let mut cy: mp_limb_t = 0;
    cy = __gmpn_add_n(rp, ap, bp, (*m).size as mp_size_t);
    cy = __gmpn_cnd_add_n(cy, rp, rp as mp_srcptr, (*m).B, (*m).size as mp_size_t);
    cy = __gmpn_cnd_add_n(cy, rp, rp as mp_srcptr, (*m).B, (*m).size as mp_size_t);
    if cy == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cy == 0\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"void _nettle_ecc_mod_add(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3640: {
        if cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void _nettle_ecc_mod_add(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn _nettle_ecc_mod_sub(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut bp: *const mp_limb_t,
) {
    let mut cy: mp_limb_t = 0;
    cy = __gmpn_sub_n(rp, ap, bp, (*m).size as mp_size_t);
    cy = __gmpn_cnd_sub_n(cy, rp, rp as mp_srcptr, (*m).Bm2m, (*m).size as mp_size_t);
    cy = __gmpn_cnd_sub_n(cy, rp, rp as mp_srcptr, (*m).B, (*m).size as mp_size_t);
    if cy == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cy == 0\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"void _nettle_ecc_mod_sub(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3754: {
        if cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void _nettle_ecc_mod_sub(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn _nettle_ecc_mod_mul_1(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut b: mp_limb_t,
) {
    let mut hi: mp_limb_t = 0;
    if b <= 0xffffffff as libc::c_uint as libc::c_ulong {} else {
        __assert_fail(
            b"b <= 0xffffffff\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"void _nettle_ecc_mod_mul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3999: {
        if b <= 0xffffffff as libc::c_uint as libc::c_ulong {} else {
            __assert_fail(
                b"b <= 0xffffffff\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_mul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    hi = __gmpn_mul_1(rp, ap, (*m).size as mp_size_t, b);
    hi = __gmpn_addmul_1(rp, (*m).B, (*m).size as mp_size_t, hi);
    if hi <= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hi <= 1\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"void _nettle_ecc_mod_mul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3926: {
        if hi <= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi <= 1\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_mul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    hi = __gmpn_cnd_add_n(hi, rp, rp as mp_srcptr, (*m).B, (*m).size as mp_size_t);
    if hi == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hi == 0\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"void _nettle_ecc_mod_mul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3865: {
        if hi == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                118 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_mul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn _nettle_ecc_mod_addmul_1(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut b: mp_limb_t,
) {
    let mut hi: mp_limb_t = 0;
    if b <= 0xffffffff as libc::c_uint as libc::c_ulong {} else {
        __assert_fail(
            b"b <= 0xffffffff\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void _nettle_ecc_mod_addmul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4184: {
        if b <= 0xffffffff as libc::c_uint as libc::c_ulong {} else {
            __assert_fail(
                b"b <= 0xffffffff\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                127 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void _nettle_ecc_mod_addmul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    hi = __gmpn_addmul_1(rp, ap, (*m).size as mp_size_t, b);
    hi = __gmpn_addmul_1(rp, (*m).B, (*m).size as mp_size_t, hi);
    if hi <= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hi <= 1\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void _nettle_ecc_mod_addmul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4112: {
        if hi <= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi <= 1\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                130 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void _nettle_ecc_mod_addmul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    hi = __gmpn_cnd_add_n(hi, rp, rp as mp_srcptr, (*m).B, (*m).size as mp_size_t);
    if hi == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hi == 0\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void _nettle_ecc_mod_addmul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4051: {
        if hi == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void _nettle_ecc_mod_addmul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn _nettle_ecc_mod_submul_1(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut b: mp_limb_t,
) {
    let mut hi: mp_limb_t = 0;
    if b <= 0xffffffff as libc::c_uint as libc::c_ulong {} else {
        __assert_fail(
            b"b <= 0xffffffff\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void _nettle_ecc_mod_submul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4367: {
        if b <= 0xffffffff as libc::c_uint as libc::c_ulong {} else {
            __assert_fail(
                b"b <= 0xffffffff\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void _nettle_ecc_mod_submul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    hi = __gmpn_submul_1(rp, ap, (*m).size as mp_size_t, b);
    hi = __gmpn_submul_1(rp, (*m).B, (*m).size as mp_size_t, hi);
    if hi <= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hi <= 1\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void _nettle_ecc_mod_submul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4295: {
        if hi <= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi <= 1\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void _nettle_ecc_mod_submul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    hi = __gmpn_cnd_sub_n(hi, rp, rp as mp_srcptr, (*m).B, (*m).size as mp_size_t);
    if hi == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hi == 0\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void _nettle_ecc_mod_submul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4235: {
        if hi == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"hi == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-arith.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void _nettle_ecc_mod_submul_1(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn _nettle_ecc_mod_mul(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut bp: *const mp_limb_t,
    mut tp: *mut mp_limb_t,
) {
    __gmpn_mul_n(tp, ap, bp, (*m).size as mp_size_t);
    ((*m).reduce).unwrap()(m, rp, tp);
}
pub unsafe extern "C" fn _nettle_ecc_mod_sqr(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut tp: *mut mp_limb_t,
) {
    __gmpn_sqr(tp, ap, (*m).size as mp_size_t);
    ((*m).reduce).unwrap()(m, rp, tp);
}
pub unsafe extern "C" fn _nettle_ecc_mod_mul_canonical(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut bp: *const mp_limb_t,
    mut tp: *mut mp_limb_t,
) {
    let mut cy: mp_limb_t = 0;
    __gmpn_mul_n(tp, ap, bp, (*m).size as mp_size_t);
    ((*m).reduce).unwrap()(m, tp.offset((*m).size as libc::c_int as isize), tp);
    cy = __gmpn_sub_n(
        rp,
        tp.offset((*m).size as libc::c_int as isize) as mp_srcptr,
        (*m).m,
        (*m).size as mp_size_t,
    );
    _nettle_cnd_copy(
        cy as libc::c_int,
        rp,
        tp.offset((*m).size as libc::c_int as isize),
        (*m).size as mp_size_t,
    );
}
pub unsafe extern "C" fn _nettle_ecc_mod_sqr_canonical(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut tp: *mut mp_limb_t,
) {
    let mut cy: mp_limb_t = 0;
    __gmpn_sqr(tp, ap, (*m).size as mp_size_t);
    ((*m).reduce).unwrap()(m, tp.offset((*m).size as libc::c_int as isize), tp);
    cy = __gmpn_sub_n(
        rp,
        tp.offset((*m).size as libc::c_int as isize) as mp_srcptr,
        (*m).m,
        (*m).size as mp_size_t,
    );
    _nettle_cnd_copy(
        cy as libc::c_int,
        rp,
        tp.offset((*m).size as libc::c_int as isize),
        (*m).size as mp_size_t,
    );
}
pub unsafe extern "C" fn _nettle_ecc_mod_pow_2k(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut xp: *const mp_limb_t,
    mut k: libc::c_uint,
    mut tp: *mut mp_limb_t,
) {
    _nettle_ecc_mod_sqr(m, rp, xp, tp);
    loop {
        k = k.wrapping_sub(1);
        if !(k > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        _nettle_ecc_mod_sqr(m, rp, rp, tp);
    };
}
pub unsafe extern "C" fn _nettle_ecc_mod_pow_2k_mul(
    mut m: *const ecc_modulo,
    mut rp: *mut mp_limb_t,
    mut xp: *const mp_limb_t,
    mut k: libc::c_uint,
    mut yp: *const mp_limb_t,
    mut tp: *mut mp_limb_t,
) {
    _nettle_ecc_mod_pow_2k(m, rp, xp, k, tp);
    _nettle_ecc_mod_mul(m, rp, rp, yp, tp);
}
