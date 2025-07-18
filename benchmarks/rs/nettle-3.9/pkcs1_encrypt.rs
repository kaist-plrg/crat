use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub unsafe extern "C" fn nettle_pkcs1_encrypt(
    mut key_size: size_t,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut length: size_t,
    mut message: *const uint8_t,
    mut m: *mut __mpz_struct,
) -> libc::c_int {
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    let mut padding: size_t = 0;
    let mut i: size_t = 0;
    if length.wrapping_add(11 as libc::c_int as libc::c_ulong) > key_size {
        return 0 as libc::c_int;
    }
    padding = key_size
        .wrapping_sub(length)
        .wrapping_sub(3 as libc::c_int as libc::c_ulong);
    if padding >= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"padding >= 8\0" as *const u8 as *const libc::c_char,
            b"pkcs1-encrypt.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"int nettle_pkcs1_encrypt(size_t, void *, nettle_random_func *, size_t, const uint8_t *, __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4824: {
        if padding >= 8 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"padding >= 8\0" as *const u8 as *const libc::c_char,
                b"pkcs1-encrypt.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"int nettle_pkcs1_encrypt(size_t, void *, nettle_random_func *, size_t, const uint8_t *, __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    tmp_em_size = key_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(key_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)),
    ) as *mut uint8_t;
    *em.offset(0 as libc::c_int as isize) = 2 as libc::c_int as uint8_t;
    random.unwrap()(random_ctx, padding, em.offset(1 as libc::c_int as isize));
    i = 0 as libc::c_int as size_t;
    while i < padding {
        if *em.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) == 0 {
            *em
                .offset(
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 1 as libc::c_int as uint8_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    *em
        .offset(
            padding.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as uint8_t;
    memcpy(
        em.offset(padding as isize).offset(2 as libc::c_int as isize)
            as *mut libc::c_void,
        message as *const libc::c_void,
        length,
    );
    nettle_mpz_set_str_256_u(
        m,
        key_size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        em,
    );
    _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
    return 1 as libc::c_int;
}
