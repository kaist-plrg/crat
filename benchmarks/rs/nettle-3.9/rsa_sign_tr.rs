use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn __gmpz_limbs_read(_: mpz_srcptr) -> mp_srcptr;
    fn __gmpz_limbs_write(_: mpz_ptr, _: mp_size_t) -> mp_ptr;
    fn __gmpz_limbs_finish(_: mpz_ptr, _: mp_size_t);
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_mpn_set_base256(
        rp: *mut mp_limb_t,
        rn: mp_size_t,
        xp: *const uint8_t,
        xn: size_t,
    );
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
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
    fn __gmpn_sec_invert(
        _: mp_ptr,
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_bitcnt_t,
        _: mp_ptr,
    ) -> libc::c_int;
    fn __gmpn_sec_invert_itch(_: mp_size_t) -> mp_size_t;
    fn _nettle_rsa_sec_compute_root_itch(key: *const rsa_private_key) -> mp_size_t;
    fn _nettle_rsa_sec_compute_root(
        key: *const rsa_private_key,
        rp: *mut mp_limb_t,
        mp: *const mp_limb_t,
        scratch: *mut mp_limb_t,
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
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
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
unsafe extern "C" fn rsa_sec_blind(
    mut pub_0: *const rsa_public_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut c: *mut mp_limb_t,
    mut ri: *mut mp_limb_t,
    mut m: *const mp_limb_t,
) {
    let mut ep: *const mp_limb_t = __gmpz_limbs_read(((*pub_0).e).as_ptr());
    let mut np: *const mp_limb_t = __gmpz_limbs_read(((*pub_0).n).as_ptr());
    let mut ebn: mp_bitcnt_t = __gmpz_sizeinbase(
        ((*pub_0).e).as_ptr(),
        2 as libc::c_int,
    );
    let mut nn: mp_size_t = __gmpz_size(((*pub_0).n).as_ptr()) as mp_size_t;
    let mut itch: size_t = 0;
    let mut i2: size_t = 0;
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tp: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_tp_size: size_t = 0;
    let mut rp: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_rp_size: size_t = 0;
    let mut r: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_r_size: size_t = 0;
    tmp_rp_size = nn as size_t;
    rp = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(nn as libc::c_ulong),
    ) as *mut mp_limb_t;
    tmp_r_size = (nn as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong);
    r = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(
                (nn as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
            ),
    ) as *mut uint8_t;
    itch = __gmpn_sec_powm_itch(nn, ebn, nn) as size_t;
    i2 = __gmpn_sec_mul_itch(nn, nn) as size_t;
    itch = if itch > i2 { itch } else { i2 };
    i2 = __gmpn_sec_div_r_itch(2 as libc::c_int as libc::c_long * nn, nn) as size_t;
    itch = if itch > i2 { itch } else { i2 };
    i2 = __gmpn_sec_invert_itch(nn) as size_t;
    itch = if itch > i2 { itch } else { i2 };
    tmp_tp_size = ((2 as libc::c_int as libc::c_long * nn) as libc::c_ulong)
        .wrapping_add(itch);
    tp = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(
                ((2 as libc::c_int as libc::c_long * nn) as libc::c_ulong)
                    .wrapping_add(itch),
            ),
    ) as *mut mp_limb_t;
    scratch = tp.offset((2 as libc::c_int as libc::c_long * nn) as isize);
    loop {
        random
            .unwrap()(
            random_ctx,
            (nn as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
            r,
        );
        _nettle_mpn_set_base256(
            rp,
            nn,
            r,
            (nn as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
        );
        __gmpn_copyi(tp, rp as mp_srcptr, nn);
        if !(__gmpn_sec_invert(
            ri,
            tp,
            np,
            nn,
            (2 as libc::c_int as libc::c_long * nn
                * (64 as libc::c_int - 0 as libc::c_int) as libc::c_long) as mp_bitcnt_t,
            scratch,
        ) == 0)
        {
            break;
        }
    }
    __gmpn_sec_powm(c, rp as mp_srcptr, nn, ep, ebn, np, nn, scratch);
    __gmpn_sec_mul(tp, c as mp_srcptr, nn, m, nn, scratch);
    __gmpn_sec_div_r(tp, 2 as libc::c_int as libc::c_long * nn, np, nn, scratch);
    __gmpn_copyi(c, tp as mp_srcptr, nn);
    _nettle_gmp_free(r as *mut libc::c_void, tmp_r_size);
    _nettle_gmp_free(rp as *mut libc::c_void, tmp_rp_size);
    _nettle_gmp_free(tp as *mut libc::c_void, tmp_tp_size);
}
unsafe extern "C" fn rsa_sec_unblind(
    mut pub_0: *const rsa_public_key,
    mut x: *mut mp_limb_t,
    mut ri: *mut mp_limb_t,
    mut c: *const mp_limb_t,
) {
    let mut np: *const mp_limb_t = __gmpz_limbs_read(((*pub_0).n).as_ptr());
    let mut nn: mp_size_t = __gmpz_size(((*pub_0).n).as_ptr()) as mp_size_t;
    let mut itch: size_t = 0;
    let mut i2: size_t = 0;
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tp: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_tp_size: size_t = 0;
    itch = __gmpn_sec_mul_itch(nn, nn) as size_t;
    i2 = __gmpn_sec_div_r_itch(nn + nn, nn) as size_t;
    itch = if itch > i2 { itch } else { i2 };
    tmp_tp_size = ((nn + nn) as libc::c_ulong).wrapping_add(itch);
    tp = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(((nn + nn) as libc::c_ulong).wrapping_add(itch)),
    ) as *mut mp_limb_t;
    scratch = tp.offset(nn as isize).offset(nn as isize);
    __gmpn_sec_mul(tp, c, nn, ri as mp_srcptr, nn, scratch);
    __gmpn_sec_div_r(tp, nn + nn, np, nn, scratch);
    __gmpn_copyi(x, tp as mp_srcptr, nn);
    _nettle_gmp_free(tp as *mut libc::c_void, tmp_tp_size);
}
unsafe extern "C" fn sec_equal(
    mut a: *const mp_limb_t,
    mut b: *const mp_limb_t,
    mut limbs: size_t,
) -> libc::c_int {
    let mut z: mp_limb_t = 0 as libc::c_int as mp_limb_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < limbs {
        ::std::ptr::write_volatile(
            &mut z as *mut mp_limb_t,
            (::std::ptr::read_volatile::<mp_limb_t>(&z as *const mp_limb_t)
                as libc::c_ulong | *a.offset(i as isize) ^ *b.offset(i as isize))
                as mp_limb_t as mp_limb_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    return (z == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn rsa_sec_check_root(
    mut pub_0: *const rsa_public_key,
    mut x: *const mp_limb_t,
    mut m: *const mp_limb_t,
) -> libc::c_int {
    let mut nn: mp_size_t = __gmpz_size(((*pub_0).n).as_ptr()) as mp_size_t;
    let mut ebn: mp_size_t = __gmpz_sizeinbase(((*pub_0).e).as_ptr(), 2 as libc::c_int)
        as mp_size_t;
    let mut np: *const mp_limb_t = __gmpz_limbs_read(((*pub_0).n).as_ptr());
    let mut ep: *const mp_limb_t = __gmpz_limbs_read(((*pub_0).e).as_ptr());
    let mut ret: libc::c_int = 0;
    let mut itch: mp_size_t = 0;
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tp: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_tp_size: size_t = 0;
    itch = __gmpn_sec_powm_itch(nn, ebn as mp_bitcnt_t, nn);
    tmp_tp_size = (nn + itch) as size_t;
    tp = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul((nn + itch) as libc::c_ulong),
    ) as *mut mp_limb_t;
    scratch = tp.offset(nn as isize);
    __gmpn_sec_powm(tp, x, nn, ep, ebn as mp_bitcnt_t, np, nn, scratch);
    ret = sec_equal(tp, m, nn as size_t);
    _nettle_gmp_free(tp as *mut libc::c_void, tmp_tp_size);
    return ret;
}
unsafe extern "C" fn cnd_mpn_zero(
    mut cnd: libc::c_int,
    mut rp: mp_ptr,
    mut n: mp_size_t,
) {
    let mut c: mp_limb_t = 0;
    let mut mask: mp_limb_t = (cnd as mp_limb_t)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        ::std::ptr::write_volatile(&mut c as *mut mp_limb_t, *rp.offset(n as isize));
        ::std::ptr::write_volatile(
            &mut c as *mut mp_limb_t,
            (::std::ptr::read_volatile::<mp_limb_t>(&c as *const mp_limb_t)
                as libc::c_ulong & mask) as mp_limb_t as mp_limb_t,
        );
        *rp.offset(n as isize) = c;
    };
}
pub unsafe extern "C" fn _nettle_rsa_sec_compute_root_tr(
    mut pub_0: *const rsa_public_key,
    mut key: *const rsa_private_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut x: *mut mp_limb_t,
    mut m: *const mp_limb_t,
) -> libc::c_int {
    let mut c: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_c_size: size_t = 0;
    let mut ri: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_ri_size: size_t = 0;
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_scratch_size: size_t = 0;
    let mut key_limb_size: size_t = 0;
    let mut ret: libc::c_int = 0;
    key_limb_size = __gmpz_size(((*pub_0).n).as_ptr());
    if ((*((*pub_0).n).as_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*((*pub_0).n).as_ptr())._mp_d).offset(0 as libc::c_int as isize)
            as libc::c_int == 0
        || ((*((*key).p).as_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
            & *((*((*key).p).as_ptr())._mp_d).offset(0 as libc::c_int as isize)
                as libc::c_int == 0
        || ((*((*key).q).as_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
            & *((*((*key).q).as_ptr())._mp_d).offset(0 as libc::c_int as isize)
                as libc::c_int == 0
    {
        __gmpn_zero(x, key_limb_size as mp_size_t);
        return 0 as libc::c_int;
    }
    if __gmpz_size(((*pub_0).n).as_ptr()) == key_limb_size {} else {
        __assert_fail(
            b"mpz_size(pub->n) == key_limb_size\0" as *const u8 as *const libc::c_char,
            b"rsa-sign-tr.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 161],
                &[libc::c_char; 161],
            >(
                b"int _nettle_rsa_sec_compute_root_tr(const struct rsa_public_key *, const struct rsa_private_key *, void *, nettle_random_func *, mp_limb_t *, const mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5020: {
        if __gmpz_size(((*pub_0).n).as_ptr()) == key_limb_size {} else {
            __assert_fail(
                b"mpz_size(pub->n) == key_limb_size\0" as *const u8
                    as *const libc::c_char,
                b"rsa-sign-tr.c\0" as *const u8 as *const libc::c_char,
                320 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 161],
                    &[libc::c_char; 161],
                >(
                    b"int _nettle_rsa_sec_compute_root_tr(const struct rsa_public_key *, const struct rsa_private_key *, void *, nettle_random_func *, mp_limb_t *, const mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    tmp_c_size = key_limb_size;
    c = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong).wrapping_mul(key_limb_size),
    ) as *mut mp_limb_t;
    tmp_ri_size = key_limb_size;
    ri = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong).wrapping_mul(key_limb_size),
    ) as *mut mp_limb_t;
    tmp_scratch_size = _nettle_rsa_sec_compute_root_itch(key) as size_t;
    scratch = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(_nettle_rsa_sec_compute_root_itch(key) as libc::c_ulong),
    ) as *mut mp_limb_t;
    rsa_sec_blind(pub_0, random_ctx, random, c, ri, m);
    _nettle_rsa_sec_compute_root(key, x, c, scratch);
    ret = rsa_sec_check_root(pub_0, x, c);
    rsa_sec_unblind(pub_0, x, ri, x);
    cnd_mpn_zero(1 as libc::c_int - ret, x, key_limb_size as mp_size_t);
    _nettle_gmp_free(scratch as *mut libc::c_void, tmp_scratch_size);
    _nettle_gmp_free(ri as *mut libc::c_void, tmp_ri_size);
    _nettle_gmp_free(c as *mut libc::c_void, tmp_c_size);
    return ret;
}
pub unsafe extern "C" fn nettle_rsa_compute_root_tr(
    mut pub_0: *const rsa_public_key,
    mut key: *const rsa_private_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut x: *mut __mpz_struct,
    mut m: *const __mpz_struct,
) -> libc::c_int {
    let mut l: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_l_size: size_t = 0;
    let mut nn: mp_size_t = __gmpz_size(((*pub_0).n).as_ptr()) as mp_size_t;
    let mut res: libc::c_int = 0;
    tmp_l_size = nn as size_t;
    l = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(nn as libc::c_ulong),
    ) as *mut mp_limb_t;
    _nettle_mpz_limbs_copy(l, m, nn);
    res = _nettle_rsa_sec_compute_root_tr(pub_0, key, random_ctx, random, l, l);
    if res != 0 {
        let mut xp: *mut mp_limb_t = __gmpz_limbs_write(x, nn);
        __gmpn_copyi(xp, l as mp_srcptr, nn);
        __gmpz_limbs_finish(x, nn);
    }
    _nettle_gmp_free(l as *mut libc::c_void, tmp_l_size);
    return res;
}
