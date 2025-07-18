use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_rshift(
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: libc::c_uint,
    ) -> mp_limb_t;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
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
    fn __gmpn_cnd_swap(_: mp_limb_t, _: *mut mp_limb_t, _: *mut mp_limb_t, _: mp_size_t);
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
unsafe extern "C" fn cnd_neg(
    mut cnd: libc::c_int,
    mut rp: *mut mp_limb_t,
    mut ap_0: *const mp_limb_t,
    mut n: mp_size_t,
) {
    let mut cy: mp_limb_t = (cnd != 0 as libc::c_int) as libc::c_int as mp_limb_t;
    let mut mask: mp_limb_t = cy.wrapping_neg();
    let mut i: mp_size_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    while i < n {
        let mut r: mp_limb_t = (*ap_0.offset(i as isize) ^ mask).wrapping_add(cy);
        cy = (r < cy) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = r;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _nettle_ecc_mod_inv(
    mut m: *const ecc_modulo,
    mut vp: *mut mp_limb_t,
    mut in_ap: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut n: mp_size_t = (*m).size as mp_size_t;
    let mut i: libc::c_uint = 0;
    if scratch != vp {} else {
        __assert_fail(
            b"ap != vp\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4166: {
        if scratch != vp {} else {
            __assert_fail(
                b"ap != vp\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *scratch
        .offset((2 as libc::c_int as libc::c_long * n) as isize)
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as mp_limb_t;
    __gmpn_zero(
        scratch
            .offset((2 as libc::c_int as libc::c_long * n) as isize)
            .offset(1 as libc::c_int as isize),
        n - 1 as libc::c_int as libc::c_long,
    );
    __gmpn_copyi(scratch.offset(n as isize), (*m).m, n);
    __gmpn_zero(vp, n);
    __gmpn_copyi(scratch, in_ap, n);
    i = ((*m).bit_size as libc::c_long
        + (64 as libc::c_int - 0 as libc::c_int) as libc::c_long * n) as libc::c_uint;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        let mut odd: mp_limb_t = 0;
        let mut swap: mp_limb_t = 0;
        let mut cy: mp_limb_t = 0;
        if *scratch.offset(n as isize).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_ulong != 0
        {} else {
            __assert_fail(
                b"bp[0] & 1\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4024: {
            if *scratch.offset(n as isize).offset(0 as libc::c_int as isize)
                & 1 as libc::c_int as libc::c_ulong != 0
            {} else {
                __assert_fail(
                    b"bp[0] & 1\0" as *const u8 as *const libc::c_char,
                    b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                    138 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        odd = *scratch.offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_ulong;
        swap = __gmpn_cnd_sub_n(
            odd,
            scratch,
            scratch as mp_srcptr,
            scratch.offset(n as isize) as mp_srcptr,
            n,
        );
        __gmpn_cnd_add_n(
            swap,
            scratch.offset(n as isize),
            scratch.offset(n as isize) as mp_srcptr,
            scratch as mp_srcptr,
            n,
        );
        cnd_neg(swap as libc::c_int, scratch, scratch, n);
        __gmpn_cnd_swap(
            swap,
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize)
                as *mut mp_limb_t,
            vp as *mut mp_limb_t,
            n,
        );
        cy = __gmpn_cnd_sub_n(
            odd,
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize),
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize) as mp_srcptr,
            vp as mp_srcptr,
            n,
        );
        cy = (cy as libc::c_ulong)
            .wrapping_sub(
                __gmpn_cnd_add_n(
                    cy,
                    scratch.offset((2 as libc::c_int as libc::c_long * n) as isize),
                    scratch.offset((2 as libc::c_int as libc::c_long * n) as isize)
                        as mp_srcptr,
                    (*m).m,
                    n,
                ),
            ) as mp_limb_t as mp_limb_t;
        if cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3762: {
            if cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"cy == 0\0" as *const u8 as *const libc::c_char,
                    b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                    148 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        cy = __gmpn_rshift(
            scratch,
            scratch as mp_srcptr,
            n,
            1 as libc::c_int as libc::c_uint,
        );
        if cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3709: {
            if cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"cy == 0\0" as *const u8 as *const libc::c_char,
                    b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                    151 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        cy = __gmpn_rshift(
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize),
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize) as mp_srcptr,
            n,
            1 as libc::c_int as libc::c_uint,
        );
        cy = __gmpn_cnd_add_n(
            cy,
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize),
            scratch.offset((2 as libc::c_int as libc::c_long * n) as isize) as mp_srcptr,
            (*m).mp1h,
            n,
        );
        if cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                154 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3606: {
            if cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"cy == 0\0" as *const u8 as *const libc::c_char,
                    b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                    154 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    if *scratch.offset(0 as libc::c_int as isize)
        | *scratch.offset((n - 1 as libc::c_int as libc::c_long) as isize)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(ap[0] | ap[n-1]) == 0\0" as *const u8 as *const libc::c_char,
            b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3511: {
        if *scratch.offset(0 as libc::c_int as isize)
            | *scratch.offset((n - 1 as libc::c_int as libc::c_long) as isize)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"(ap[0] | ap[n-1]) == 0\0" as *const u8 as *const libc::c_char,
                b"ecc-mod-inv.c\0" as *const u8 as *const libc::c_char,
                156 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 97],
                    &[libc::c_char; 97],
                >(
                    b"void _nettle_ecc_mod_inv(const struct ecc_modulo *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
