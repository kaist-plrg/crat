use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_com(_: mpz_ptr, _: mpz_srcptr);
    fn __gmpz_import(
        _: mpz_ptr,
        _: size_t,
        _: libc::c_int,
        _: size_t,
        _: libc::c_int,
        _: size_t,
        _: *const libc::c_void,
    );
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_mul_2exp(_: mpz_ptr, _: mpz_srcptr, _: mp_bitcnt_t);
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn __gmpz_sub(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
#[inline]
unsafe extern "C" fn __gmpz_getlimbn(
    mut __gmp_z: mpz_srcptr,
    mut __gmp_n: mp_size_t,
) -> mp_limb_t {
    let mut __gmp_result: mp_limb_t = 0 as libc::c_int as mp_limb_t;
    if ((__gmp_n >= 0 as libc::c_int as libc::c_long
        && __gmp_n
            < (if (*__gmp_z)._mp_size >= 0 as libc::c_int {
                (*__gmp_z)._mp_size
            } else {
                -(*__gmp_z)._mp_size
            }) as libc::c_long) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        __gmp_result = *((*__gmp_z)._mp_d).offset(__gmp_n as isize);
    }
    return __gmp_result;
}
pub unsafe extern "C" fn nettle_mpz_sizeinbase_256_s(
    mut x: *const __mpz_struct,
) -> size_t {
    if (if (*x)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*x)._mp_size > 0 as libc::c_int) as libc::c_int
    }) >= 0 as libc::c_int
    {
        return (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (__gmpz_sizeinbase(x, 2 as libc::c_int))
                    .wrapping_div(8 as libc::c_int as libc::c_ulong),
            )
    } else {
        let mut size: size_t = 0;
        let mut c: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(c.as_mut_ptr());
        __gmpz_com(c.as_mut_ptr(), x);
        size = (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (__gmpz_sizeinbase(c.as_mut_ptr() as mpz_srcptr, 2 as libc::c_int))
                    .wrapping_div(8 as libc::c_int as libc::c_ulong),
            );
        __gmpz_clear(c.as_mut_ptr());
        return size;
    };
}
pub unsafe extern "C" fn nettle_mpz_sizeinbase_256_u(
    mut x: *const __mpz_struct,
) -> size_t {
    return (__gmpz_sizeinbase(x, 2 as libc::c_int))
        .wrapping_add(7 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn nettle_mpz_to_octets(
    mut length: size_t,
    mut s: *mut uint8_t,
    mut x: *const __mpz_struct,
    mut sign: uint8_t,
) {
    let mut dst: *mut uint8_t = s
        .offset(length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut size: size_t = __gmpz_size(x);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        let mut limb: mp_limb_t = __gmpz_getlimbn(x, i as mp_size_t);
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while length != 0 && j < ::std::mem::size_of::<mp_limb_t>() as libc::c_ulong {
            let fresh0 = dst;
            dst = dst.offset(-1);
            *fresh0 = (sign as libc::c_ulong
                ^ limb & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            limb >>= 8 as libc::c_int;
            length = length.wrapping_sub(1);
            length;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if length != 0 {
        memset(s as *mut libc::c_void, sign as libc::c_int, length);
    }
}
pub unsafe extern "C" fn nettle_mpz_get_str_256(
    mut length: size_t,
    mut s: *mut uint8_t,
    mut x: *const __mpz_struct,
) {
    if length == 0 {
        if (if (*x)._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*x)._mp_size > 0 as libc::c_int) as libc::c_int
        }) == 0
        {} else {
            __assert_fail(
                b"!mpz_sgn(x)\0" as *const u8 as *const libc::c_char,
                b"bignum.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void nettle_mpz_get_str_256(size_t, uint8_t *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3916: {
            if (if (*x)._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*x)._mp_size > 0 as libc::c_int) as libc::c_int
            }) == 0
            {} else {
                __assert_fail(
                    b"!mpz_sgn(x)\0" as *const u8 as *const libc::c_char,
                    b"bignum.c\0" as *const u8 as *const libc::c_char,
                    114 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"void nettle_mpz_get_str_256(size_t, uint8_t *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return;
    }
    if (if (*x)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*x)._mp_size > 0 as libc::c_int) as libc::c_int
    }) >= 0 as libc::c_int
    {
        if nettle_mpz_sizeinbase_256_u(x) <= length {} else {
            __assert_fail(
                b"nettle_mpz_sizeinbase_256_u(x) <= length\0" as *const u8
                    as *const libc::c_char,
                b"bignum.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void nettle_mpz_get_str_256(size_t, uint8_t *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3849: {
            if nettle_mpz_sizeinbase_256_u(x) <= length {} else {
                __assert_fail(
                    b"nettle_mpz_sizeinbase_256_u(x) <= length\0" as *const u8
                        as *const libc::c_char,
                    b"bignum.c\0" as *const u8 as *const libc::c_char,
                    120 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"void nettle_mpz_get_str_256(size_t, uint8_t *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        nettle_mpz_to_octets(length, s, x, 0 as libc::c_int as uint8_t);
    } else {
        let mut c: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(c.as_mut_ptr());
        __gmpz_com(c.as_mut_ptr(), x);
        if nettle_mpz_sizeinbase_256_u(c.as_mut_ptr() as *const __mpz_struct) <= length
        {} else {
            __assert_fail(
                b"nettle_mpz_sizeinbase_256_u(c) <= length\0" as *const u8
                    as *const libc::c_char,
                b"bignum.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void nettle_mpz_get_str_256(size_t, uint8_t *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3772: {
            if nettle_mpz_sizeinbase_256_u(c.as_mut_ptr() as *const __mpz_struct)
                <= length
            {} else {
                __assert_fail(
                    b"nettle_mpz_sizeinbase_256_u(c) <= length\0" as *const u8
                        as *const libc::c_char,
                    b"bignum.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"void nettle_mpz_get_str_256(size_t, uint8_t *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        nettle_mpz_to_octets(
            length,
            s,
            c.as_mut_ptr() as *const __mpz_struct,
            0xff as libc::c_int as uint8_t,
        );
        __gmpz_clear(c.as_mut_ptr());
    };
}
pub unsafe extern "C" fn nettle_mpz_set_str_256_u(
    mut x: *mut __mpz_struct,
    mut length: size_t,
    mut s: *const uint8_t,
) {
    __gmpz_import(
        x,
        length,
        1 as libc::c_int,
        1 as libc::c_int as size_t,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        s as *const libc::c_void,
    );
}
pub unsafe extern "C" fn nettle_mpz_init_set_str_256_u(
    mut x: *mut __mpz_struct,
    mut length: size_t,
    mut s: *const uint8_t,
) {
    __gmpz_init(x);
    __gmpz_import(
        x,
        length,
        1 as libc::c_int,
        1 as libc::c_int as size_t,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        s as *const libc::c_void,
    );
}
pub unsafe extern "C" fn nettle_mpz_set_str_256_s(
    mut x: *mut __mpz_struct,
    mut length: size_t,
    mut s: *const uint8_t,
) {
    if length == 0 {
        __gmpz_set_ui(x, 0 as libc::c_int as libc::c_ulong);
        return;
    }
    __gmpz_import(
        x,
        length,
        1 as libc::c_int,
        1 as libc::c_int as size_t,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        s as *const libc::c_void,
    );
    if *s.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        let mut t: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init_set_ui(t.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
        __gmpz_mul_2exp(
            t.as_mut_ptr(),
            t.as_mut_ptr() as mpz_srcptr,
            length.wrapping_mul(8 as libc::c_int as libc::c_ulong),
        );
        __gmpz_sub(x, x as mpz_srcptr, t.as_mut_ptr() as mpz_srcptr);
        __gmpz_clear(t.as_mut_ptr());
    }
}
pub unsafe extern "C" fn nettle_mpz_init_set_str_256_s(
    mut x: *mut __mpz_struct,
    mut length: size_t,
    mut s: *const uint8_t,
) {
    __gmpz_init(x);
    nettle_mpz_set_str_256_s(x, length, s);
}
