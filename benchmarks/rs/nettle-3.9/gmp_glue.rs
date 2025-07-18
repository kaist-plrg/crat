use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmp_get_memory_functions(
        _: *mut Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: *mut Option::<
            unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
        >,
        _: *mut Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
    );
    fn __gmpz_limbs_read(_: mpz_srcptr) -> mp_srcptr;
    fn __gmpz_limbs_write(_: mpz_ptr, _: mp_size_t) -> mp_ptr;
    fn __gmpz_limbs_finish(_: mpz_ptr, _: mp_size_t);
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[inline]
unsafe extern "C" fn __gmpz_size(mut __gmp_z: mpz_srcptr) -> size_t {
    return (if (*__gmp_z)._mp_size >= 0 as libc::c_int {
        (*__gmp_z)._mp_size
    } else {
        -(*__gmp_z)._mp_size
    }) as size_t;
}
pub unsafe extern "C" fn _nettle_sec_zero_p(
    mut ap: *const mp_limb_t,
    mut n: mp_size_t,
) -> libc::c_int {
    let mut w: mp_limb_t = 0;
    let mut i: mp_size_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    ::std::ptr::write_volatile(&mut w as *mut mp_limb_t, 0 as libc::c_int as mp_limb_t);
    while i < n {
        ::std::ptr::write_volatile(
            &mut w as *mut mp_limb_t,
            (::std::ptr::read_volatile::<mp_limb_t>(&w as *const mp_limb_t)
                as libc::c_ulong | *ap.offset(i as isize)) as mp_limb_t as mp_limb_t,
        );
        i += 1;
        i;
    }
    return (w == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn _nettle_mpz_limbs_copy(
    mut xp: *mut mp_limb_t,
    mut x: mpz_srcptr,
    mut n: mp_size_t,
) {
    let mut xn: mp_size_t = __gmpz_size(x) as mp_size_t;
    if xn <= n {} else {
        __assert_fail(
            b"xn <= n\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void _nettle_mpz_limbs_copy(mp_limb_t *, mpz_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_4488: {
        if xn <= n {} else {
            __assert_fail(
                b"xn <= n\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void _nettle_mpz_limbs_copy(mp_limb_t *, mpz_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    __gmpn_copyi(xp, __gmpz_limbs_read(x), xn);
    if xn < n {
        __gmpn_zero(xp.offset(xn as isize), n - xn);
    }
}
pub unsafe extern "C" fn _nettle_mpz_set_n(
    mut r: *mut __mpz_struct,
    mut xp: *const mp_limb_t,
    mut xn: mp_size_t,
) {
    __gmpn_copyi(__gmpz_limbs_write(r, xn), xp, xn);
    __gmpz_limbs_finish(r, xn);
}
pub unsafe extern "C" fn _nettle_mpn_set_base256(
    mut rp: *mut mp_limb_t,
    mut rn: mp_size_t,
    mut xp: *const uint8_t,
    mut xn: size_t,
) {
    let mut xi: size_t = 0;
    let mut out: mp_limb_t = 0;
    let mut bits: libc::c_uint = 0;
    xi = xn;
    bits = 0 as libc::c_int as libc::c_uint;
    out = bits as mp_limb_t;
    while xi > 0 as libc::c_int as libc::c_ulong && rn > 0 as libc::c_int as libc::c_long
    {
        xi = xi.wrapping_sub(1);
        let mut in_0: mp_limb_t = *xp.offset(xi as isize) as mp_limb_t;
        out |= in_0 << bits & !(0 as libc::c_int as mp_limb_t) >> 0 as libc::c_int;
        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
        if bits >= (64 as libc::c_int - 0 as libc::c_int) as libc::c_uint {
            let fresh0 = rp;
            rp = rp.offset(1);
            *fresh0 = out;
            rn -= 1;
            rn;
            bits = bits
                .wrapping_sub((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint);
            out = in_0 >> (8 as libc::c_int as libc::c_uint).wrapping_sub(bits);
        }
    }
    if rn > 0 as libc::c_int as libc::c_long {
        let fresh1 = rp;
        rp = rp.offset(1);
        *fresh1 = out;
        rn -= 1;
        if rn > 0 as libc::c_int as libc::c_long {
            __gmpn_zero(rp, rn);
        }
    }
}
pub unsafe extern "C" fn _nettle_mpn_set_base256_le(
    mut rp: *mut mp_limb_t,
    mut rn: mp_size_t,
    mut xp: *const uint8_t,
    mut xn: size_t,
) {
    let mut xi: size_t = 0;
    let mut out: mp_limb_t = 0;
    let mut bits: libc::c_uint = 0;
    xi = 0 as libc::c_int as size_t;
    bits = 0 as libc::c_int as libc::c_uint;
    out = bits as mp_limb_t;
    while xi < xn && rn > 0 as libc::c_int as libc::c_long {
        let fresh2 = xi;
        xi = xi.wrapping_add(1);
        let mut in_0: mp_limb_t = *xp.offset(fresh2 as isize) as mp_limb_t;
        out |= in_0 << bits & !(0 as libc::c_int as mp_limb_t) >> 0 as libc::c_int;
        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
        if bits >= (64 as libc::c_int - 0 as libc::c_int) as libc::c_uint {
            let fresh3 = rp;
            rp = rp.offset(1);
            *fresh3 = out;
            rn -= 1;
            rn;
            bits = bits
                .wrapping_sub((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint);
            out = in_0 >> (8 as libc::c_int as libc::c_uint).wrapping_sub(bits);
        }
    }
    if rn > 0 as libc::c_int as libc::c_long {
        let fresh4 = rp;
        rp = rp.offset(1);
        *fresh4 = out;
        rn -= 1;
        if rn > 0 as libc::c_int as libc::c_long {
            __gmpn_zero(rp, rn);
        }
    }
}
pub unsafe extern "C" fn _nettle_mpn_get_base256(
    mut rp: *mut uint8_t,
    mut rn: size_t,
    mut xp: *const mp_limb_t,
    mut xn: mp_size_t,
) {
    let mut bits: libc::c_uint = 0;
    let mut in_0: mp_limb_t = 0;
    in_0 = 0 as libc::c_int as mp_limb_t;
    bits = in_0 as libc::c_uint;
    while xn > 0 as libc::c_int as libc::c_long && rn > 0 as libc::c_int as libc::c_ulong
    {
        if bits >= 8 as libc::c_int as libc::c_uint {
            rn = rn.wrapping_sub(1);
            *rp.offset(rn as isize) = in_0 as uint8_t;
            in_0 >>= 8 as libc::c_int;
            bits = bits.wrapping_sub(8 as libc::c_int as libc::c_uint);
        } else {
            let mut old: uint8_t = in_0 as uint8_t;
            let fresh5 = xp;
            xp = xp.offset(1);
            in_0 = *fresh5;
            xn -= 1;
            xn;
            rn = rn.wrapping_sub(1);
            *rp.offset(rn as isize) = (old as libc::c_ulong | in_0 << bits) as uint8_t;
            in_0 >>= (8 as libc::c_int as libc::c_uint).wrapping_sub(bits);
            bits = bits
                .wrapping_add(
                    (64 as libc::c_int - 0 as libc::c_int - 8 as libc::c_int)
                        as libc::c_uint,
                );
        }
    }
    while rn > 0 as libc::c_int as libc::c_ulong {
        rn = rn.wrapping_sub(1);
        *rp.offset(rn as isize) = in_0 as uint8_t;
        in_0 >>= 8 as libc::c_int;
    }
}
pub unsafe extern "C" fn _nettle_mpn_get_base256_le(
    mut rp: *mut uint8_t,
    mut rn: size_t,
    mut xp: *const mp_limb_t,
    mut xn: mp_size_t,
) {
    let mut bits: libc::c_uint = 0;
    let mut in_0: mp_limb_t = 0;
    in_0 = 0 as libc::c_int as mp_limb_t;
    bits = in_0 as libc::c_uint;
    while xn > 0 as libc::c_int as libc::c_long && rn > 0 as libc::c_int as libc::c_ulong
    {
        if bits >= 8 as libc::c_int as libc::c_uint {
            let fresh6 = rp;
            rp = rp.offset(1);
            *fresh6 = in_0 as uint8_t;
            rn = rn.wrapping_sub(1);
            rn;
            in_0 >>= 8 as libc::c_int;
            bits = bits.wrapping_sub(8 as libc::c_int as libc::c_uint);
        } else {
            let mut old: uint8_t = in_0 as uint8_t;
            let fresh7 = xp;
            xp = xp.offset(1);
            in_0 = *fresh7;
            xn -= 1;
            xn;
            let fresh8 = rp;
            rp = rp.offset(1);
            *fresh8 = (old as libc::c_ulong | in_0 << bits) as uint8_t;
            rn = rn.wrapping_sub(1);
            rn;
            in_0 >>= (8 as libc::c_int as libc::c_uint).wrapping_sub(bits);
            bits = bits
                .wrapping_add(
                    (64 as libc::c_int - 0 as libc::c_int - 8 as libc::c_int)
                        as libc::c_uint,
                );
        }
    }
    while rn > 0 as libc::c_int as libc::c_ulong {
        let fresh9 = rp;
        rp = rp.offset(1);
        *fresh9 = in_0 as uint8_t;
        rn = rn.wrapping_sub(1);
        rn;
        in_0 >>= 8 as libc::c_int;
    }
}
pub unsafe extern "C" fn _nettle_gmp_alloc_limbs(mut n: mp_size_t) -> *mut mp_limb_t {
    let mut alloc_func: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void> = None;
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"mp_limb_t *_nettle_gmp_alloc_limbs(mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5145: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                285 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"mp_limb_t *_nettle_gmp_alloc_limbs(mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    __gmp_get_memory_functions(
        &mut alloc_func,
        0
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
            >,
        0 as *mut Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
    );
    return alloc_func
        .unwrap()(
        (n as size_t).wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
}
pub unsafe extern "C" fn _nettle_gmp_free_limbs(
    mut p: *mut mp_limb_t,
    mut n: mp_size_t,
) {
    let mut free_func: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()> = None;
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void _nettle_gmp_free_limbs(mp_limb_t *, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5260: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                295 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void _nettle_gmp_free_limbs(mp_limb_t *, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if !p.is_null() {} else {
        __assert_fail(
            b"p != 0\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void _nettle_gmp_free_limbs(mp_limb_t *, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5220: {
        if !p.is_null() {} else {
            __assert_fail(
                b"p != 0\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                296 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void _nettle_gmp_free_limbs(mp_limb_t *, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    __gmp_get_memory_functions(
        0 as *mut Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        0
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
            >,
        &mut free_func,
    );
    free_func
        .unwrap()(
        p as *mut libc::c_void,
        (n as size_t).wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    );
}
pub unsafe extern "C" fn _nettle_gmp_alloc(mut n: size_t) -> *mut libc::c_void {
    let mut alloc_func: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void> = None;
    if n > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            306 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void *_nettle_gmp_alloc(size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5324: {
        if n > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void *_nettle_gmp_alloc(size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    __gmp_get_memory_functions(
        &mut alloc_func,
        0
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
            >,
        0 as *mut Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
    );
    return alloc_func.unwrap()(n);
}
pub unsafe extern "C" fn _nettle_gmp_free(mut p: *mut libc::c_void, mut n: size_t) {
    let mut free_func: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()> = None;
    if n > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void _nettle_gmp_free(void *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5433: {
        if n > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                317 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void _nettle_gmp_free(void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if !p.is_null() {} else {
        __assert_fail(
            b"p != 0\0" as *const u8 as *const libc::c_char,
            b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void _nettle_gmp_free(void *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5394: {
        if !p.is_null() {} else {
            __assert_fail(
                b"p != 0\0" as *const u8 as *const libc::c_char,
                b"gmp-glue.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void _nettle_gmp_free(void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    __gmp_get_memory_functions(
        0 as *mut Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        0
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
            >,
        &mut free_func,
    );
    free_func.unwrap()(p, n);
}
