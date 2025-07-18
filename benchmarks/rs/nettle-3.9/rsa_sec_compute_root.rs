use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_limbs_read(_: mpz_srcptr) -> mp_srcptr;
    fn __gmpn_add_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_cnd_add_n(
        _: mp_limb_t,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_srcptr,
        _: mp_size_t,
    ) -> mp_limb_t;
    fn __gmpn_sec_add_1(
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_limb_t,
        _: mp_ptr,
    ) -> mp_limb_t;
    fn __gmpn_sec_add_1_itch(_: mp_size_t) -> mp_size_t;
    fn __gmpn_sec_mul(
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_ptr,
    );
    fn __gmpn_sec_mul_itch(_: mp_size_t, _: mp_size_t) -> mp_size_t;
    fn __gmpn_sec_powm(
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_srcptr,
        _: mp_bitcnt_t,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_ptr,
    );
    fn __gmpn_sec_powm_itch(_: mp_size_t, _: mp_bitcnt_t, _: mp_size_t) -> mp_size_t;
    fn __gmpn_sec_div_r(_: mp_ptr, _: mp_size_t, _: mp_srcptr, _: mp_size_t, _: mp_ptr);
    fn __gmpn_sec_div_r_itch(_: mp_size_t, _: mp_size_t) -> mp_size_t;
}
pub type size_t = libc::c_ulong;
pub type mp_limb_t = libc::c_ulong;
pub type mp_bitcnt_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_private_key {
    pub size: size_t,
    pub d: mpz_t,
    pub p: mpz_t,
    pub q: mpz_t,
    pub a: mpz_t,
    pub b: mpz_t,
    pub c: mpz_t,
}
#[inline]
unsafe extern "C" fn __gmpz_size(mut __gmp_z: mpz_srcptr) -> size_t {
    return (if (*__gmp_z)._mp_size >= 0 as libc::c_int {
        (*__gmp_z)._mp_size
    } else {
        -(*__gmp_z)._mp_size
    }) as size_t;
}
unsafe extern "C" fn sec_mul_itch(mut an: mp_size_t, mut bn: mp_size_t) -> mp_size_t {
    if an >= bn {
        return __gmpn_sec_mul_itch(an, bn)
    } else {
        return __gmpn_sec_mul_itch(bn, an)
    };
}
unsafe extern "C" fn sec_mul(
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut an: mp_size_t,
    mut bp: *const mp_limb_t,
    mut bn: mp_size_t,
    mut scratch: *mut mp_limb_t,
) {
    if an >= bn {
        __gmpn_sec_mul(rp, ap, an, bp, bn, scratch);
    } else {
        __gmpn_sec_mul(rp, bp, bn, ap, an, scratch);
    };
}
unsafe extern "C" fn sec_mod_mul_itch(
    mut an: mp_size_t,
    mut bn: mp_size_t,
    mut mn: mp_size_t,
) -> mp_size_t {
    let mut mul_itch: mp_size_t = sec_mul_itch(an, bn);
    let mut mod_itch: mp_size_t = __gmpn_sec_div_r_itch(an + bn, mn);
    return if mul_itch > mod_itch { mul_itch } else { mod_itch };
}
unsafe extern "C" fn sec_mod_mul(
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut an: mp_size_t,
    mut bp: *const mp_limb_t,
    mut bn: mp_size_t,
    mut mp: *const mp_limb_t,
    mut mn: mp_size_t,
    mut scratch: *mut mp_limb_t,
) {
    if an + bn >= mn {} else {
        __assert_fail(
            b"an + bn >= mn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 133],
                &[libc::c_char; 133],
            >(
                b"void sec_mod_mul(mp_limb_t *, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4562: {
        if an + bn >= mn {} else {
            __assert_fail(
                b"an + bn >= mn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 133],
                    &[libc::c_char; 133],
                >(
                    b"void sec_mod_mul(mp_limb_t *, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sec_mul(rp, ap, an, bp, bn, scratch);
    __gmpn_sec_div_r(rp, an + bn, mp, mn, scratch);
}
unsafe extern "C" fn sec_powm_itch(
    mut bn: mp_size_t,
    mut en: mp_size_t,
    mut mn: mp_size_t,
) -> mp_size_t {
    let mut mod_itch: mp_size_t = bn + __gmpn_sec_div_r_itch(bn, mn);
    let mut pow_itch: mp_size_t = mn
        + __gmpn_sec_powm_itch(
            mn,
            (en * (64 as libc::c_int - 0 as libc::c_int) as libc::c_long) as mp_bitcnt_t,
            mn,
        );
    return if mod_itch > pow_itch { mod_itch } else { pow_itch };
}
unsafe extern "C" fn sec_powm(
    mut rp: *mut mp_limb_t,
    mut bp: *const mp_limb_t,
    mut bn: mp_size_t,
    mut ep: *const mp_limb_t,
    mut en: mp_size_t,
    mut mp: *const mp_limb_t,
    mut mn: mp_size_t,
    mut scratch: *mut mp_limb_t,
) {
    if bn >= mn {} else {
        __assert_fail(
            b"bn >= mn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 130],
                &[libc::c_char; 130],
            >(
                b"void sec_powm(mp_limb_t *, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4796: {
        if bn >= mn {} else {
            __assert_fail(
                b"bn >= mn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 130],
                    &[libc::c_char; 130],
                >(
                    b"void sec_powm(mp_limb_t *, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if en <= mn {} else {
        __assert_fail(
            b"en <= mn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 130],
                &[libc::c_char; 130],
            >(
                b"void sec_powm(mp_limb_t *, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4756: {
        if en <= mn {} else {
            __assert_fail(
                b"en <= mn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 130],
                    &[libc::c_char; 130],
                >(
                    b"void sec_powm(mp_limb_t *, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, const mp_limb_t *, mp_size_t, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    __gmpn_copyi(scratch, bp, bn);
    __gmpn_sec_div_r(scratch, bn, mp, mn, scratch.offset(bn as isize));
    __gmpn_sec_powm(
        rp,
        scratch as mp_srcptr,
        mn,
        ep,
        (en * (64 as libc::c_int - 0 as libc::c_int) as libc::c_long) as mp_bitcnt_t,
        mp,
        mn,
        scratch.offset(mn as isize),
    );
}
pub unsafe extern "C" fn _nettle_rsa_sec_compute_root_itch(
    mut key: *const rsa_private_key,
) -> mp_size_t {
    let mut nn: mp_size_t = ((*key).size)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        as mp_size_t;
    let mut pn: mp_size_t = __gmpz_size(((*key).p).as_ptr()) as mp_size_t;
    let mut qn: mp_size_t = __gmpz_size(((*key).q).as_ptr()) as mp_size_t;
    let mut an: mp_size_t = __gmpz_size(((*key).a).as_ptr()) as mp_size_t;
    let mut bn: mp_size_t = __gmpz_size(((*key).b).as_ptr()) as mp_size_t;
    let mut cn: mp_size_t = __gmpz_size(((*key).c).as_ptr()) as mp_size_t;
    let mut powm_p_itch: mp_size_t = sec_powm_itch(nn, an, pn);
    let mut powm_q_itch: mp_size_t = sec_powm_itch(nn, bn, qn);
    let mut mod_mul_itch: mp_size_t = cn + (if pn > qn { pn } else { qn })
        + sec_mod_mul_itch((if pn > qn { pn } else { qn }), cn, pn);
    let mut mul_itch: mp_size_t = sec_mul_itch(qn, pn);
    let mut add_1_itch: mp_size_t = __gmpn_sec_add_1_itch(nn - qn);
    let mut itch: mp_size_t = pn + qn
        + (if mul_itch > add_1_itch { mul_itch } else { add_1_itch });
    itch = if itch > powm_p_itch { itch } else { powm_p_itch };
    itch = if itch > powm_q_itch { itch } else { powm_q_itch };
    itch = if itch > mod_mul_itch { itch } else { mod_mul_itch };
    return pn + qn + itch;
}
pub unsafe extern "C" fn _nettle_rsa_sec_compute_root(
    mut key: *const rsa_private_key,
    mut rp: *mut mp_limb_t,
    mut mp: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut nn: mp_size_t = ((*key).size)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        as mp_size_t;
    let mut pp: *const mp_limb_t = __gmpz_limbs_read(((*key).p).as_ptr());
    let mut qp: *const mp_limb_t = __gmpz_limbs_read(((*key).q).as_ptr());
    let mut pn: mp_size_t = __gmpz_size(((*key).p).as_ptr()) as mp_size_t;
    let mut qn: mp_size_t = __gmpz_size(((*key).q).as_ptr()) as mp_size_t;
    let mut an: mp_size_t = __gmpz_size(((*key).a).as_ptr()) as mp_size_t;
    let mut bn: mp_size_t = __gmpz_size(((*key).b).as_ptr()) as mp_size_t;
    let mut cn: mp_size_t = __gmpz_size(((*key).c).as_ptr()) as mp_size_t;
    let mut r_mod_p: *mut mp_limb_t = scratch;
    let mut r_mod_q: *mut mp_limb_t = scratch.offset(pn as isize);
    let mut scratch_out: *mut mp_limb_t = r_mod_q.offset(qn as isize);
    let mut cy: mp_limb_t = 0;
    if pn <= nn {} else {
        __assert_fail(
            b"pn <= nn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5020: {
        if pn <= nn {} else {
            __assert_fail(
                b"pn <= nn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if qn <= nn {} else {
        __assert_fail(
            b"qn <= nn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4982: {
        if qn <= nn {} else {
            __assert_fail(
                b"qn <= nn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if an <= pn {} else {
        __assert_fail(
            b"an <= pn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4944: {
        if an <= pn {} else {
            __assert_fail(
                b"an <= pn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if bn <= qn {} else {
        __assert_fail(
            b"bn <= qn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4906: {
        if bn <= qn {} else {
            __assert_fail(
                b"bn <= qn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                171 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if cn <= pn {} else {
        __assert_fail(
            b"cn <= pn\0" as *const u8 as *const libc::c_char,
            b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4867: {
        if cn <= pn {} else {
            __assert_fail(
                b"cn <= pn\0" as *const u8 as *const libc::c_char,
                b"rsa-sec-compute-root.c\0" as *const u8 as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"void _nettle_rsa_sec_compute_root(const struct rsa_private_key *, mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sec_powm(
        r_mod_p,
        mp,
        nn,
        __gmpz_limbs_read(((*key).a).as_ptr()),
        an,
        pp,
        pn,
        scratch_out,
    );
    sec_powm(
        r_mod_q,
        mp,
        nn,
        __gmpz_limbs_read(((*key).b).as_ptr()),
        bn,
        qp,
        qn,
        scratch_out,
    );
    sec_mod_mul(
        scratch_out,
        r_mod_p,
        pn,
        __gmpz_limbs_read(((*key).c).as_ptr()),
        cn,
        pp,
        pn,
        scratch_out.offset(cn as isize).offset(pn as isize),
    );
    __gmpn_copyi(r_mod_p, scratch_out as mp_srcptr, pn);
    sec_mod_mul(
        scratch_out,
        r_mod_q,
        qn,
        __gmpz_limbs_read(((*key).c).as_ptr()),
        cn,
        pp,
        pn,
        scratch_out.offset(cn as isize).offset(qn as isize),
    );
    cy = __gmpn_sub_n(r_mod_p, r_mod_p as mp_srcptr, scratch_out as mp_srcptr, pn);
    __gmpn_cnd_add_n(cy, r_mod_p, r_mod_p as mp_srcptr, pp, pn);
    sec_mul(
        scratch_out,
        qp,
        qn,
        r_mod_p,
        pn,
        scratch_out.offset(pn as isize).offset(qn as isize),
    );
    cy = __gmpn_add_n(rp, scratch_out as mp_srcptr, r_mod_q as mp_srcptr, qn);
    __gmpn_sec_add_1(
        rp.offset(qn as isize),
        scratch_out.offset(qn as isize) as mp_srcptr,
        nn - qn,
        cy,
        scratch_out.offset(pn as isize).offset(qn as isize),
    );
}
